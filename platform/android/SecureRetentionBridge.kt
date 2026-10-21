package coda.duet

import android.content.Context
import android.os.Build
import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyProperties
import android.util.AtomicFile
import org.json.JSONArray
import org.json.JSONObject
import java.io.File
import java.security.KeyStore
import java.security.MessageDigest
import javax.crypto.Cipher
import javax.crypto.KeyGenerator
import javax.crypto.SecretKey
import javax.crypto.spec.GCMParameterSpec

/** Android Keystore protects AES keys; only authenticated ciphertext enters noBackupFilesDir.
 * No SharedPreferences/plaintext fallback. A damaged item/key is an error, never an empty draft.
 * Requires API 28+ for unlocked-device-only keys. Physical device qualification is separate.
 */
object SecureRetentionBridge {
    private const val MAX_BYTES = 1_048_576
    private fun name(key: String) = MessageDigest.getInstance("SHA-256")
        .digest(key.toByteArray(Charsets.UTF_8)).joinToString("") { "%02x".format(it) }

    private fun key(alias: String, create: Boolean): SecretKey {
        check(Build.VERSION.SDK_INT >= 28)
        val store = KeyStore.getInstance("AndroidKeyStore").apply { load(null) }
        val existing = store.getKey(alias, null)
        if (existing is SecretKey) return existing
        check(create) // Key loss must not mint a new key and silently erase the old draft.
        return KeyGenerator.getInstance(KeyProperties.KEY_ALGORITHM_AES, "AndroidKeyStore").run {
            init(KeyGenParameterSpec.Builder(alias, KeyProperties.PURPOSE_ENCRYPT or KeyProperties.PURPOSE_DECRYPT)
                .setBlockModes(KeyProperties.BLOCK_MODE_GCM)
                .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_NONE)
                .setRandomizedEncryptionRequired(true)
                .setUnlockedDeviceRequired(true)
                .build())
            generateKey()
        }
    }

    @JvmStatic @Synchronized
    fun call(context: Context, input: String): String {
      return try {
        val request = JSONObject(input)
        if (request.getString("op") == "copy_link") {
            val clipboard = context.getSystemService(Context.CLIPBOARD_SERVICE) as android.content.ClipboardManager
            val value = request.getString("value")
            val clip = android.content.ClipData.newPlainText("Duet private link", value)
            clip.description.extras = android.os.PersistableBundle().apply { putBoolean("android.content.extra.IS_SENSITIVE", true) }
            clipboard.setPrimaryClip(clip)
            android.os.Handler(android.os.Looper.getMainLooper()).postDelayed({
                if (clipboard.primaryClip?.getItemAt(0)?.text?.toString() == value) clipboard.clearPrimaryClip()
            }, 120_000)
            return JSONObject().put("ok", true).put("value", JSONObject.NULL).toString()
        }
        if (request.getString("op") == "share_pdf") {
            val array = request.getJSONArray("bytes"); check(array.length() <= 20_000_000)
            val bytes = ByteArray(array.length()) { index -> val byte = array.getInt(index); check(byte in 0..255); byte.toByte() }
            DoctorPdfProvider.share(context, bytes)
            return JSONObject().put("ok", true).put("value", JSONObject.NULL).toString()
        }
        if (request.getString("op").startsWith("cleanup_")) {
            // Device-protected, nonsecret deny bit: never put identities/tokens/PHI here.
            check(Build.VERSION.SDK_INT >= 28)
            val directBoot = context.createDeviceProtectedStorageContext()
            val preferences = directBoot.getSharedPreferences("duet-cleanup-fence", Context.MODE_PRIVATE)
            val value: Any = when (request.getString("op")) {
                "cleanup_read" -> preferences.getBoolean("pending", false)
                "cleanup_write" -> { if (request.getBoolean("pending")) DoctorPdfProvider.clear(context); check(preferences.edit().putBoolean("pending", request.getBoolean("pending")).commit()); JSONObject.NULL }
                else -> error("Unknown fence operation")
            }
            return JSONObject().put("ok", true).put("value", value).toString()
        }
        val ownerKey = request.getString("key")
        check(ownerKey.isNotEmpty() && ownerKey.length < 1024)
        val digest = name(ownerKey)
        val alias = "coda.duet.retention.$digest"
        val directory = File(context.noBackupFilesDir, "duet-protected-v1")
        check(directory.exists() || directory.mkdirs())
        val file = AtomicFile(File(directory, digest))
        val aad = ownerKey.toByteArray(Charsets.UTF_8)
        val value: Any = when (request.getString("op")) {
            "read" -> {
                val packed = try { file.readFully() } catch (e: java.io.FileNotFoundException) {
                    if (file.baseFile.exists()) throw e else null
                }
                if (packed == null) JSONObject.NULL else {
                    check(packed.size in 29..(MAX_BYTES + 28))
                    val cipher = Cipher.getInstance("AES/GCM/NoPadding")
                    cipher.init(Cipher.DECRYPT_MODE, key(alias, false), GCMParameterSpec(128, packed.copyOfRange(0, 12)))
                    cipher.updateAAD(aad)
                    JSONArray(cipher.doFinal(packed.copyOfRange(12, packed.size)).map { it.toInt() and 255 })
                }
            }
            "write" -> {
                val array = request.getJSONArray("bytes")
                check(array.length() <= MAX_BYTES)
                val plain = ByteArray(array.length()) { index ->
                    val byte = array.getInt(index); check(byte in 0..255); byte.toByte()
                }
                try {
                    val cipher = Cipher.getInstance("AES/GCM/NoPadding")
                    cipher.init(Cipher.ENCRYPT_MODE, key(alias, !file.baseFile.exists()))
                    cipher.updateAAD(aad)
                    val encrypted = cipher.iv + cipher.doFinal(plain)
                    val stream = file.startWrite()
                    try { stream.write(encrypted); file.finishWrite(stream) }
                    catch (error: Exception) { file.failWrite(stream); throw error }
                } finally { plain.fill(0) }
                JSONObject.NULL
            }
            "remove" -> {
                // Delete ciphertext before key; if erasure fails, preserve the key for retry.
                file.delete()
                check(!file.baseFile.exists())
                KeyStore.getInstance("AndroidKeyStore").apply { load(null); deleteEntry(alias) }
                JSONObject.NULL
            }
            else -> error("Unsupported protected storage operation")
        }
        JSONObject().put("ok", true).put("value", value).toString()
    } catch (_: Exception) {
        // Never log provider tokens, patient prose, or raw keystore exceptions.
        JSONObject().put("ok", false).toString()
      }
    }
}
