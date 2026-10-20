package coda.duet

import android.Manifest
import android.app.AlarmManager
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Build
import android.provider.Settings
import androidx.activity.ComponentActivity
import androidx.activity.result.contract.ActivityResultContracts
import androidx.core.app.NotificationCompat
import androidx.core.app.NotificationManagerCompat
import androidx.core.content.ContextCompat
import java.util.Calendar
import org.json.JSONObject

/**
 * One local insight alert, matching the iPhone coordinator as far as the
 * existing alert API allows. A remote token is not created here.
 */
object NotificationBridge {
    private const val channelId = "health.duet.insights"
    private const val requestPrefix = "health.duet.notification.daily."
    private const val prefsName = "coda.duet.notification"
    private const val plannedKey = "coda.duet.insight.planned-cycle"
    private const val enabledKey = "coda.duet.insight.enabled"
    private const val askedKey = "coda.duet.notification.asked"
    private const val alertId = 0xD0E7
    private const val title = "New insights from Duet"
    private const val body = "See what may be worth noticing in your health information."

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
    fun lastPlanned(context: Context): String =
        prefs(context).getString(plannedKey, "").orEmpty()

    @JvmStatic
    fun insightsEnabled(context: Context): String =
        if (prefs(context).getBoolean(enabledKey, false)) "true" else "false"

    @JvmStatic
    fun setInsights(context: Context, enabled: String): String {
        if (enabled != "true") {
            prefs(context).edit().putBoolean(enabledKey, false).apply()
            cancelInsights(context)
            return "false"
        }
        if (status(context) == "not_determined") {
            requestPermission(context)
        }
        if (status(context) != "allowed") {
            return "false"
        }
        prefs(context).edit().putBoolean(enabledKey, true).apply()
        return "true"
    }

    @JvmStatic
    fun cancelInsights(context: Context) {
        val alarm = context.getSystemService(AlarmManager::class.java) ?: return
        pending(context, null, PendingIntent.FLAG_NO_CREATE)?.let { intent ->
            alarm.cancel(intent)
            intent.cancel()
        }
        NotificationManagerCompat.from(context).cancel(alertId)
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

    @JvmStatic
    fun schedule(context: Context, payload: String) {
        if (status(context) != "allowed" || insightsEnabled(context) != "true") {
            return
        }
        val json = try {
            JSONObject(payload)
        } catch (_: Throwable) {
            return
        }
        val identifier = json.optString("identifier")
        val cycleId = json.optString("cycle_id")
        if (!identifier.startsWith(requestPrefix) || cycleId.isEmpty()) {
            return
        }
        val whenMillis = Calendar.getInstance().apply {
            set(Calendar.YEAR, json.optInt("year"))
            set(Calendar.MONTH, json.optInt("month") - 1)
            set(Calendar.DAY_OF_MONTH, json.optInt("day"))
            set(Calendar.HOUR_OF_DAY, json.optInt("hour"))
            set(Calendar.MINUTE, json.optInt("minute"))
            set(Calendar.SECOND, 0)
            set(Calendar.MILLISECOND, 0)
        }.timeInMillis
        ensureChannel(context)
        val alarm = context.getSystemService(AlarmManager::class.java) ?: return
        val intent = pending(context, cycleId, PendingIntent.FLAG_UPDATE_CURRENT) ?: return
        val scheduled = try {
            alarm.setExactAndAllowWhileIdle(AlarmManager.RTC_WAKEUP, whenMillis, intent)
            true
        } catch (_: Throwable) {
            try {
                alarm.setAndAllowWhileIdle(AlarmManager.RTC_WAKEUP, whenMillis, intent)
                true
            } catch (error: Throwable) {
                android.util.Log.e("DuetNotify", "insight alert was not scheduled", error)
                false
            }
        }
        if (scheduled) {
            prefs(context).edit().putString(plannedKey, cycleId).apply()
        }
    }

    fun show(context: Context, cycleId: String?) {
        ensureChannel(context)
        val notification = NotificationCompat.Builder(context, channelId)
            .setSmallIcon(android.R.drawable.ic_dialog_info)
            .setContentTitle(title)
            .setContentText(body)
            .setCategory(NotificationCompat.CATEGORY_REMINDER)
            .setAutoCancel(true)
            .build()
        try {
            NotificationManagerCompat.from(context).notify(alertId, notification)
        } catch (error: Throwable) {
            android.util.Log.e("DuetNotify", "insight alert was not shown", error)
        }
        if (!cycleId.isNullOrEmpty()) {
            prefs(context).edit().putString(plannedKey, cycleId).apply()
        }
    }

    private fun prefs(context: Context) =
        context.getSharedPreferences(prefsName, Context.MODE_PRIVATE)

    private fun ensureChannel(context: Context) {
        if (Build.VERSION.SDK_INT < 26) {
            return
        }
        val manager = context.getSystemService(NotificationManager::class.java) ?: return
        if (manager.getNotificationChannel(channelId) != null) {
            return
        }
        manager.createNotificationChannel(
            NotificationChannel(channelId, "Insights", NotificationManager.IMPORTANCE_DEFAULT),
        )
    }

    private fun pending(context: Context, cycleId: String?, flags: Int): PendingIntent? {
        val intent = Intent(context, InsightAlertReceiver::class.java)
        if (cycleId != null) {
            intent.putExtra("cycle_id", cycleId)
        }
        val mode = flags or PendingIntent.FLAG_IMMUTABLE
        return if (flags == PendingIntent.FLAG_NO_CREATE) {
            PendingIntent.getBroadcast(context, alertId, intent, mode)
        } else {
            PendingIntent.getBroadcast(context, alertId, intent, mode)
        }
    }
}

/** Posts the one daily insight alert when its alarm fires. */
class InsightAlertReceiver : BroadcastReceiver() {
    override fun onReceive(context: Context, intent: Intent) {
        NotificationBridge.show(context, intent.getStringExtra("cycle_id"))
    }
}
