package coda.duet

import android.content.ContentProvider
import android.content.ContentValues
import android.content.Context
import android.content.Intent
import android.database.Cursor
import android.database.MatrixCursor
import android.net.Uri
import android.os.Handler
import android.os.Looper
import android.os.ParcelFileDescriptor
import android.provider.OpenableColumns
import java.util.UUID

/** Temporary read-only PDF sharing. Bytes never enter disk/cache/backup. Process death revokes access. */
class DoctorPdfProvider : ContentProvider() {
    companion object {
        private data class Export(val bytes: ByteArray, val expires: Long)
        private val exports = mutableMapOf<String, Export>()
        @Synchronized fun clear(context: Context) {
            for ((id, value) in exports) {
                context.revokeUriPermission(Uri.parse("content://${context.packageName}.duet.pdf/$id"), Intent.FLAG_GRANT_READ_URI_PERMISSION)
                value.bytes.fill(0)
            }
            exports.clear()
        }
        @Synchronized fun share(context: Context, bytes: ByteArray) {
            clear(context)
            val id = UUID.randomUUID().toString()
            exports[id] = Export(bytes, android.os.SystemClock.elapsedRealtime() + 300_000)
            val uri = Uri.parse("content://${context.packageName}.duet.pdf/$id")
            Handler(Looper.getMainLooper()).post {
                val send = Intent(Intent.ACTION_SEND).setType("application/pdf")
                    .putExtra(Intent.EXTRA_STREAM, uri)
                    .addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
                context.startActivity(Intent.createChooser(send, "Share For my doctor").addFlags(Intent.FLAG_ACTIVITY_NEW_TASK))
            }
            Handler(Looper.getMainLooper()).postDelayed({ synchronized(this) {
                exports.remove(id)?.bytes?.fill(0)
                context.revokeUriPermission(uri, Intent.FLAG_GRANT_READ_URI_PERMISSION)
            } }, 300_000)
        }
        @Synchronized private fun bytes(uri: Uri): ByteArray {
            val entry = exports[uri.lastPathSegment] ?: throw java.io.FileNotFoundException()
            if (entry.expires <= android.os.SystemClock.elapsedRealtime()) throw java.io.FileNotFoundException()
            return entry.bytes.copyOf()
        }
    }
    override fun onCreate() = true
    override fun getType(uri: Uri) = "application/pdf"
    override fun query(uri: Uri, projection: Array<out String>?, selection: String?, selectionArgs: Array<out String>?, sortOrder: String?): Cursor {
        val data = bytes(uri)
        return MatrixCursor(arrayOf(OpenableColumns.DISPLAY_NAME, OpenableColumns.SIZE)).apply { addRow(arrayOf("for-my-doctor.pdf", data.size)); data.fill(0) }
    }
    override fun openFile(uri: Uri, mode: String): ParcelFileDescriptor {
        if (mode != "r") throw java.io.FileNotFoundException()
        val data = bytes(uri)
        val pipe = ParcelFileDescriptor.createPipe()
        Thread {
            try { ParcelFileDescriptor.AutoCloseOutputStream(pipe[1]).use { it.write(data) } }
            finally { data.fill(0) }
        }.start()
        return pipe[0]
    }
    override fun insert(uri: Uri, values: ContentValues?): Uri? = throw SecurityException()
    override fun delete(uri: Uri, selection: String?, selectionArgs: Array<out String>?): Int = throw SecurityException()
    override fun update(uri: Uri, values: ContentValues?, selection: String?, selectionArgs: Array<out String>?): Int = throw SecurityException()
}
