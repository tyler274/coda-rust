package coda.duet

import android.Manifest
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Build
import android.provider.Settings
import androidx.activity.ComponentActivity
import androidx.activity.result.contract.ActivityResultContracts
import androidx.core.app.NotificationManagerCompat
import androidx.core.content.ContextCompat

/** Notification permission, cached FCM token bridge, and sign-out alert cleanup for `duet-notify`. */
object NotificationBridge {
    private const val prefsName = "coda.duet.notification"
    private const val askedKey = "coda.duet.notification.asked"
    private const val tokenKey = "coda.duet.notification.fcm_token"

    /** Called by the FirebaseMessagingService when Android issues or rotates an FCM token. */
    @JvmStatic
    fun storePushToken(context: Context, token: String) {
        val trimmed = token.trim()
        if (trimmed.isEmpty()) return
        prefs(context).edit().putString(tokenKey, trimmed).apply()
    }

    @JvmStatic
    fun pushToken(context: Context): String =
        prefs(context).getString(tokenKey, "")?.trim().orEmpty()

    @JvmStatic
    fun clearOnSignOut(context: Context): String = try {
        val hadToken = pushToken(context).isNotEmpty()
        prefs(context).edit().remove(tokenKey).apply()
        NotificationManagerCompat.from(context).cancelAll()
        if (hadToken) "cleared_all" else "cleared_alerts_only"
    } catch (error: Throwable) {
        android.util.Log.e("DuetNotify", "sign-out notification cleanup failed", error)
        ""
    }

    @JvmStatic
    fun status(context: Context): String = try {
        if (Build.VERSION.SDK_INT >= 33) {
            val granted = ContextCompat.checkSelfPermission(
                context,
                Manifest.permission.POST_NOTIFICATIONS,
            ) == PackageManager.PERMISSION_GRANTED
            when {
                granted -> "allowed"
                prefs(context).getBoolean(askedKey, false) -> "denied"
                else -> "not_determined"
            }
        } else if (NotificationManagerCompat.from(context).areNotificationsEnabled()) {
            "allowed"
        } else {
            "denied"
        }
    } catch (_: Throwable) {
        "unreadable"
    }

    @JvmStatic
    fun requestPermission(context: Context) {
        if (Build.VERSION.SDK_INT < 33 || status(context) != "not_determined") {
            return
        }
        val activity = context as? ComponentActivity ?: return
        prefs(context).edit().putBoolean(askedKey, true).apply()
        activity.runOnUiThread {
            try {
                val launcher = activity.activityResultRegistry.register(
                    "coda.duet.notify.${System.nanoTime()}",
                    ActivityResultContracts.RequestPermission(),
                ) { }
                launcher.launch(Manifest.permission.POST_NOTIFICATIONS)
            } catch (error: Throwable) {
                android.util.Log.e("DuetNotify", "notification permission request failed", error)
            }
        }
    }

    @JvmStatic
    fun openSettings(context: Context) {
        val intent = Intent(Settings.ACTION_APP_NOTIFICATION_SETTINGS).apply {
            putExtra(Settings.EXTRA_APP_PACKAGE, context.packageName)
            addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
        }
        try {
            context.startActivity(intent)
        } catch (error: Throwable) {
            android.util.Log.e("DuetNotify", "notification settings could not be opened", error)
        }
    }

    private fun prefs(context: Context) =
        context.getSharedPreferences(prefsName, Context.MODE_PRIVATE)
}
