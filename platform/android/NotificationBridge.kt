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

/** Notification permission for `duet-notify`. A remote token is not created here. */
object NotificationBridge {
    private const val prefsName = "coda.duet.notification"
    private const val askedKey = "coda.duet.notification.asked"

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
