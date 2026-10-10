package coda.duet

import android.content.Context
import androidx.activity.ComponentActivity
import androidx.health.connect.client.HealthConnectClient
import androidx.health.connect.client.PermissionController
import androidx.health.connect.client.permission.HealthPermission
import androidx.health.connect.client.records.ActiveCaloriesBurnedRecord
import androidx.health.connect.client.records.BasalBodyTemperatureRecord
import androidx.health.connect.client.records.BloodGlucoseRecord
import androidx.health.connect.client.records.BloodPressureRecord
import androidx.health.connect.client.records.BodyFatRecord
import androidx.health.connect.client.records.BodyTemperatureRecord
import androidx.health.connect.client.records.DistanceRecord
import androidx.health.connect.client.records.ExerciseSessionRecord
import androidx.health.connect.client.records.HeightRecord
import androidx.health.connect.client.records.HydrationRecord
import androidx.health.connect.client.records.LeanBodyMassRecord
import androidx.health.connect.client.records.NutritionRecord
import androidx.health.connect.client.records.OxygenSaturationRecord
import androidx.health.connect.client.records.RestingHeartRateRecord
import androidx.health.connect.client.records.SleepSessionRecord
import androidx.health.connect.client.records.StepsRecord
import androidx.health.connect.client.records.Vo2MaxRecord
import androidx.health.connect.client.records.WeightRecord
import androidx.health.connect.client.request.ReadRecordsRequest
import androidx.health.connect.client.time.TimeRangeFilter
import java.time.Instant
import java.time.LocalDate
import java.time.ZoneId
import kotlin.reflect.KClass
import kotlinx.coroutines.runBlocking
import org.json.JSONArray
import org.json.JSONObject

/**
 * Reads granted Health Connect records. A type that was not granted, or that
 * has no stored value, is left out. This bridge does not invent a sample.
 */
object HealthConnectBridge {
    private val recordTypes: List<KClass<out androidx.health.connect.client.records.Record>> = listOf(
        StepsRecord::class,
        ActiveCaloriesBurnedRecord::class,
        ExerciseSessionRecord::class,
        DistanceRecord::class,
        NutritionRecord::class,
        HydrationRecord::class,
        RestingHeartRateRecord::class,
        WeightRecord::class,
        SleepSessionRecord::class,
        BloodPressureRecord::class,
        BodyFatRecord::class,
        LeanBodyMassRecord::class,
        HeightRecord::class,
        BloodGlucoseRecord::class,
        BodyTemperatureRecord::class,
        BasalBodyTemperatureRecord::class,
        OxygenSaturationRecord::class,
        Vo2MaxRecord::class,
    )

    @JvmStatic
    fun status(context: Context): String = try {
        when (HealthConnectClient.getSdkStatus(context)) {
            HealthConnectClient.SDK_UNAVAILABLE -> "missing"
            HealthConnectClient.SDK_UNAVAILABLE_PROVIDER_UPDATE_REQUIRED -> "update"
            HealthConnectClient.SDK_AVAILABLE -> {
                val granted = runBlocking {
                    HealthConnectClient.getOrCreate(context).permissionController.getGrantedPermissions()
                }
                if (granted.any { permission -> permissions().contains(permission) }) "connected" else "permission"
            }
            else -> "missing"
        }
    } catch (_: Throwable) {
        "unread"
    }

    @JvmStatic
    fun readings(context: Context): String = try {
        val granted = runBlocking {
            HealthConnectClient.getOrCreate(context).permissionController.getGrantedPermissions()
        }
        val held = permissions().filter { granted.contains(it) }
        val rows = runBlocking { readGranted(context, held.toSet()) }
        JSONObject()
            .put("granted", JSONArray(held))
            .put("rows", rows)
            .toString()
    } catch (_: Throwable) {
        """{"granted":[],"rows":[]}"""
    }

    @JvmStatic
    fun requestPermission(context: Context) {
        val activity = context as? ComponentActivity
        if (activity == null) {
            android.util.Log.e("DuetHealth", "Health Connect permission needs the Duet activity")
            return
        }
        activity.runOnUiThread {
            try {
                val launcher = activity.activityResultRegistry.register(
                    "coda.duet.health.${System.nanoTime()}",
                    PermissionController.createRequestPermissionResultContract(),
                ) { }
                launcher.launch(permissions())
            } catch (error: Throwable) {
                android.util.Log.e("DuetHealth", "Health Connect permission request failed", error)
            }
        }
    }

    private fun permissions(): Set<String> = recordTypes.map { HealthPermission.getReadPermission(it) }.toSet()

    private suspend fun readGranted(context: Context, granted: Set<String>): JSONArray {
        val client = HealthConnectClient.getOrCreate(context)
        val zone = ZoneId.systemDefault()
        val start = LocalDate.now(zone).atStartOfDay(zone).toInstant()
        val end = Instant.now()
        val rows = JSONArray()
        if (!end.isAfter(start)) return rows
        val day = TimeRangeFilter.between(start, end)
        fun allow(type: KClass<out androidx.health.connect.client.records.Record>) =
            granted.contains(HealthPermission.getReadPermission(type))

        if (allow(StepsRecord::class)) {
            val total = sum(client, day, StepsRecord::class) { it.count }
            add(rows, "Steps", total, "steps")
        }
        if (allow(DistanceRecord::class)) {
            val meters = sumDouble(client, day, DistanceRecord::class) { it.distance.inMeters }
            add(rows, "Distance", meters, "m")
        }
        if (allow(ActiveCaloriesBurnedRecord::class)) {
            val kcal = sumDouble(client, day, ActiveCaloriesBurnedRecord::class) { it.energy.inKilocalories }
            add(rows, "Active energy", kcal, "kcal")
        }
        if (allow(HydrationRecord::class)) {
            val liters = sumDouble(client, day, HydrationRecord::class) { it.volume.inLiters }
            add(rows, "Water", liters, "L")
        }
        if (allow(NutritionRecord::class)) {
            val kcal = sumDouble(client, day, NutritionRecord::class) { it.energy?.inKilocalories ?: 0.0 }
            add(rows, "Dietary energy", kcal, "kcal")
        }
        if (allow(ExerciseSessionRecord::class)) {
            val minutes = sum(client, day, ExerciseSessionRecord::class) { minutesBetween(it.startTime, it.endTime) }
            add(rows, "Exercise", minutes, "min")
        }
        if (allow(SleepSessionRecord::class)) {
            val minutes = sum(client, day, SleepSessionRecord::class) { minutesBetween(it.startTime, it.endTime) }
            add(rows, "Sleep", minutes, "min")
        }
        if (allow(RestingHeartRateRecord::class)) {
            latest(client, RestingHeartRateRecord::class)?.let { add(rows, "Resting heart rate", it.beatsPerMinute, "bpm") }
        }
        if (allow(WeightRecord::class)) {
            latest(client, WeightRecord::class)?.let { add(rows, "Weight", it.weight.inKilograms, "kg") }
        }
        if (allow(HeightRecord::class)) {
            latest(client, HeightRecord::class)?.let { add(rows, "Height", it.height.inMeters, "m") }
        }
        if (allow(BodyFatRecord::class)) {
            latest(client, BodyFatRecord::class)?.let { add(rows, "Body fat", it.percentage.value, "%") }
        }
        if (allow(LeanBodyMassRecord::class)) {
            latest(client, LeanBodyMassRecord::class)?.let { add(rows, "Lean body mass", it.mass.inKilograms, "kg") }
        }
        if (allow(BloodGlucoseRecord::class)) {
            latest(client, BloodGlucoseRecord::class)?.let { add(rows, "Blood glucose", it.level.inMilligramsPerDeciliter, "mg/dL") }
        }
        if (allow(BodyTemperatureRecord::class)) {
            latest(client, BodyTemperatureRecord::class)?.let { add(rows, "Body temperature", it.temperature.inCelsius, "C") }
        }
        if (allow(BasalBodyTemperatureRecord::class)) {
            latest(client, BasalBodyTemperatureRecord::class)?.let { add(rows, "Basal body temperature", it.temperature.inCelsius, "C") }
        }
        if (allow(OxygenSaturationRecord::class)) {
            latest(client, OxygenSaturationRecord::class)?.let { add(rows, "Oxygen saturation", it.percentage.value, "%") }
        }
        if (allow(Vo2MaxRecord::class)) {
            latest(client, Vo2MaxRecord::class)?.let { add(rows, "VO2 max", it.vo2MillilitersPerMinuteKilogram, "mL/kg/min") }
        }
        if (allow(BloodPressureRecord::class)) {
            latest(client, BloodPressureRecord::class)?.let {
                add(rows, "Blood pressure", "${it.systolic.inMillimetersOfMercury.toLong()}/${it.diastolic.inMillimetersOfMercury.toLong()}", "mmHg")
            }
        }
        return rows
    }

    private fun minutesBetween(start: Instant, end: Instant): Long =
        java.time.Duration.between(start, end).toMinutes().coerceAtLeast(0)

    private suspend fun <T : androidx.health.connect.client.records.Record> sum(
        client: HealthConnectClient,
        day: TimeRangeFilter,
        type: KClass<T>,
        value: (T) -> Long,
    ): Long = readAll(client, day, type).sumOf(value)

    private suspend fun <T : androidx.health.connect.client.records.Record> sumDouble(
        client: HealthConnectClient,
        day: TimeRangeFilter,
        type: KClass<T>,
        value: (T) -> Double,
    ): Double = readAll(client, day, type).sumOf(value)

    private suspend fun <T : androidx.health.connect.client.records.Record> latest(
        client: HealthConnectClient,
        type: KClass<T>,
    ): T? = client.readRecords(
        ReadRecordsRequest(
            recordType = type,
            timeRangeFilter = TimeRangeFilter.before(Instant.now()),
            ascendingOrder = false,
            pageSize = 1,
        ),
    ).records.firstOrNull()

    private suspend fun <T : androidx.health.connect.client.records.Record> readAll(
        client: HealthConnectClient,
        day: TimeRangeFilter,
        type: KClass<T>,
    ): List<T> = client.readRecords(
        ReadRecordsRequest(recordType = type, timeRangeFilter = day, pageSize = 1000),
    ).records

    private fun add(rows: JSONArray, name: String, value: Any?, unit: String) {
        if (value == null) return
        val text = when (value) {
            is Double -> if (value == 0.0) return else trimNumber(value)
            is Float -> if (value == 0f) return else trimNumber(value.toDouble())
            is Long -> if (value == 0L) return else value.toString()
            is Int -> if (value == 0) return else value.toString()
            else -> value.toString()
        }
        if (text.isEmpty() || text == "0") return
        rows.put(JSONObject().put("name", name).put("value", text).put("unit", unit))
    }

    private fun trimNumber(value: Double): String {
        val rounded = kotlin.math.round(value * 10.0) / 10.0
        return if (rounded % 1.0 == 0.0) rounded.toLong().toString() else rounded.toString()
    }
}
