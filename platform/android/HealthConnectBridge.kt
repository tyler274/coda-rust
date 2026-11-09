package coda.duet

import android.content.Context
import androidx.activity.ComponentActivity
import androidx.health.connect.client.HealthConnectClient
import androidx.health.connect.client.PermissionController
import androidx.health.connect.client.changes.DeletionChange
import androidx.health.connect.client.changes.UpsertionChange
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
import androidx.health.connect.client.request.ChangesTokenRequest
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
 * has no stored value, is left out. Totals and latest samples stay split by
 * the app that wrote them. This bridge does not invent a sample.
 */
object HealthConnectBridge {
    // Extension point for the Health Connect type list. The types below are
    // the ones this bridge already reads. Further types wait on the list
    // Tyler is agreeing with Dev (ENG-22) before they are added here.
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
            .put("granted", namesOf(rows))
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

    // ---- Type-independent changes feed (duet_health::native_adapters) ----
    //
    // Rust owns the scope gate (disabled until ENG-22 settles the type list),
    // the committed token and coverage. These two calls only page the feed and
    // report typed errors. No new permission is declared here.

    /** `{"ok": "<token>"}` for the JSON array of record type simple names. */
    @JvmStatic
    fun getChangesToken(context: Context, typesJson: String): String = reply {
        val names = JSONArray(typesJson)
        val types = (0 until names.length()).mapNotNull { index ->
            recordTypes.firstOrNull { it.simpleName == names.getString(index) }
        }.toSet()
        if (types.isEmpty()) return@reply errorJson("unsupported")
        val token = runBlocking {
            HealthConnectClient.getOrCreate(context).getChangesToken(ChangesTokenRequest(recordTypes = types))
        }
        JSONObject().put("ok", token)
    }

    /**
     * One `getChanges(token)` page: `{"ok": {"upserts", "deleted",
     * "next_token", "has_more", "token_expired"}}`. Upserts carry the record
     * type, identity, times, offsets and `dataOrigin.packageName`. The exact
     * value is left empty until the type-specific extraction is agreed; Rust
     * reports such records as not extracted rather than uploading them.
     */
    @JvmStatic
    fun getChanges(context: Context, token: String): String = reply {
        val response = runBlocking { HealthConnectClient.getOrCreate(context).getChanges(token) }
        val upserts = JSONArray()
        val deleted = JSONArray()
        for (change in response.changes) {
            when (change) {
                is UpsertionChange -> upserts.put(upsertJson(change.record))
                is DeletionChange -> deleted.put(change.recordId)
            }
        }
        JSONObject().put(
            "ok",
            JSONObject()
                .put("upserts", upserts)
                .put("deleted", deleted)
                .put("next_token", response.nextChangesToken)
                .put("has_more", response.hasMore)
                .put("token_expired", response.changesTokenExpired),
        )
    }

    private fun upsertJson(record: androidx.health.connect.client.records.Record): JSONObject {
        val json = JSONObject()
            .put("record_type", record::class.simpleName ?: "")
            .put("id", record.metadata.id)
            .put("value", "")
            .put("writer_id", record.metadata.dataOrigin.packageName)
        record.metadata.device?.let { device ->
            listOfNotNull(device.manufacturer, device.model).joinToString(" ").takeIf { it.isNotEmpty() }?.let {
                json.put("device", it)
            }
        }
        span(record)?.let { (start, end) -> json.put("start", start).put("end", end) }
        json.put(
            "metadata",
            JSONObject()
                .put("last_modified", record.metadata.lastModifiedTime.toString())
                .put("client_record_id", record.metadata.clientRecordId ?: JSONObject.NULL)
                .put("client_record_version", record.metadata.clientRecordVersion),
        )
        return json
    }

    // Health Connect keeps InstantaneousRecord and IntervalRecord internal, so
    // each record type this bridge reads names its own time fields.
    private fun span(record: androidx.health.connect.client.records.Record): Pair<String, String>? {
        fun at(time: Instant, offset: java.time.ZoneOffset?) =
            offsetTime(time, offset).let { it to it }
        fun between(start: Instant, startOffset: java.time.ZoneOffset?, end: Instant, endOffset: java.time.ZoneOffset?) =
            offsetTime(start, startOffset) to offsetTime(end, endOffset)
        return when (record) {
            is BasalBodyTemperatureRecord -> at(record.time, record.zoneOffset)
            is BloodGlucoseRecord -> at(record.time, record.zoneOffset)
            is BloodPressureRecord -> at(record.time, record.zoneOffset)
            is BodyFatRecord -> at(record.time, record.zoneOffset)
            is BodyTemperatureRecord -> at(record.time, record.zoneOffset)
            is HeightRecord -> at(record.time, record.zoneOffset)
            is LeanBodyMassRecord -> at(record.time, record.zoneOffset)
            is OxygenSaturationRecord -> at(record.time, record.zoneOffset)
            is RestingHeartRateRecord -> at(record.time, record.zoneOffset)
            is Vo2MaxRecord -> at(record.time, record.zoneOffset)
            is WeightRecord -> at(record.time, record.zoneOffset)
            is ActiveCaloriesBurnedRecord -> between(record.startTime, record.startZoneOffset, record.endTime, record.endZoneOffset)
            is DistanceRecord -> between(record.startTime, record.startZoneOffset, record.endTime, record.endZoneOffset)
            is ExerciseSessionRecord -> between(record.startTime, record.startZoneOffset, record.endTime, record.endZoneOffset)
            is HydrationRecord -> between(record.startTime, record.startZoneOffset, record.endTime, record.endZoneOffset)
            is NutritionRecord -> between(record.startTime, record.startZoneOffset, record.endTime, record.endZoneOffset)
            is SleepSessionRecord -> between(record.startTime, record.startZoneOffset, record.endTime, record.endZoneOffset)
            is StepsRecord -> between(record.startTime, record.startZoneOffset, record.endTime, record.endZoneOffset)
            else -> null
        }
    }

    private fun offsetTime(instant: Instant, offset: java.time.ZoneOffset?): String =
        if (offset == null) instant.toString() else instant.atOffset(offset).toString()

    private inline fun reply(block: () -> JSONObject): String = try {
        block().toString()
    } catch (error: SecurityException) {
        errorJson("permission_denied").toString()
    } catch (error: android.os.RemoteException) {
        errorJson("store_unavailable").toString()
    } catch (error: IllegalStateException) {
        errorJson("store_unavailable").toString()
    } catch (error: Throwable) {
        JSONObject().put("error", JSONObject().put("code", "failed").put("message", error.toString())).toString()
    }

    private fun errorJson(code: String): JSONObject = JSONObject().put("error", JSONObject().put("code", code))

    private fun permissions(): Set<String> = recordTypes.map { HealthPermission.getReadPermission(it) }.toSet()

    private fun namesOf(rows: JSONArray): JSONArray {
        val names = JSONArray()
        for (index in 0 until rows.length()) {
            names.put(rows.getJSONObject(index).getString("name"))
        }
        return names
    }

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
            emitSum(rows, "Steps", "steps", readAll(client, day, StepsRecord::class)) { it.count.toDouble() }
        }
        if (allow(DistanceRecord::class)) {
            emitSum(rows, "Distance", "m", readAll(client, day, DistanceRecord::class)) { it.distance.inMeters }
        }
        if (allow(ActiveCaloriesBurnedRecord::class)) {
            emitSum(rows, "Active energy", "kcal", readAll(client, day, ActiveCaloriesBurnedRecord::class)) {
                it.energy.inKilocalories
            }
        }
        if (allow(HydrationRecord::class)) {
            emitSum(rows, "Water", "L", readAll(client, day, HydrationRecord::class)) { it.volume.inLiters }
        }
        if (allow(NutritionRecord::class)) {
            emitSum(rows, "Dietary energy", "kcal", readAll(client, day, NutritionRecord::class)) {
                it.energy?.inKilocalories ?: 0.0
            }
        }
        if (allow(ExerciseSessionRecord::class)) {
            emitSum(rows, "Exercise", "min", readAll(client, day, ExerciseSessionRecord::class)) {
                minutesBetween(it.startTime, it.endTime).toDouble()
            }
        }
        if (allow(SleepSessionRecord::class)) {
            emitSum(rows, "Sleep", "min", readAll(client, day, SleepSessionRecord::class)) {
                minutesBetween(it.startTime, it.endTime).toDouble()
            }
        }
        if (allow(RestingHeartRateRecord::class)) {
            emitLatest(rows, "Resting heart rate", "bpm", readLatest(client, RestingHeartRateRecord::class)) {
                it.beatsPerMinute
            }
        }
        if (allow(WeightRecord::class)) {
            emitLatest(rows, "Weight", "kg", readLatest(client, WeightRecord::class)) { it.weight.inKilograms }
        }
        if (allow(HeightRecord::class)) {
            emitLatest(rows, "Height", "m", readLatest(client, HeightRecord::class)) { it.height.inMeters }
        }
        if (allow(BodyFatRecord::class)) {
            emitLatest(rows, "Body fat", "%", readLatest(client, BodyFatRecord::class)) { it.percentage.value }
        }
        if (allow(LeanBodyMassRecord::class)) {
            emitLatest(rows, "Lean body mass", "kg", readLatest(client, LeanBodyMassRecord::class)) {
                it.mass.inKilograms
            }
        }
        if (allow(BloodGlucoseRecord::class)) {
            emitLatest(rows, "Blood glucose", "mg/dL", readLatest(client, BloodGlucoseRecord::class)) {
                it.level.inMilligramsPerDeciliter
            }
        }
        if (allow(BodyTemperatureRecord::class)) {
            emitLatest(rows, "Body temperature", "C", readLatest(client, BodyTemperatureRecord::class)) {
                it.temperature.inCelsius
            }
        }
        if (allow(BasalBodyTemperatureRecord::class)) {
            emitLatest(rows, "Basal body temperature", "C", readLatest(client, BasalBodyTemperatureRecord::class)) {
                it.temperature.inCelsius
            }
        }
        if (allow(OxygenSaturationRecord::class)) {
            emitLatest(rows, "Oxygen saturation", "%", readLatest(client, OxygenSaturationRecord::class)) {
                it.percentage.value
            }
        }
        if (allow(Vo2MaxRecord::class)) {
            emitLatest(rows, "VO2 max", "mL/kg/min", readLatest(client, Vo2MaxRecord::class)) {
                it.vo2MillilitersPerMinuteKilogram
            }
        }
        if (allow(BloodPressureRecord::class)) {
            emitLatest(rows, "Blood pressure", "mmHg", readLatest(client, BloodPressureRecord::class)) {
                "${it.systolic.inMillimetersOfMercury.toLong()}/${it.diastolic.inMillimetersOfMercury.toLong()}"
            }
        }
        return rows
    }

    private fun minutesBetween(start: Instant, end: Instant): Long =
        java.time.Duration.between(start, end).toMinutes().coerceAtLeast(0)

    private fun <T : androidx.health.connect.client.records.Record> emitSum(
        rows: JSONArray,
        name: String,
        unit: String,
        records: List<T>,
        value: (T) -> Double,
    ) {
        val totals = linkedMapOf<String, Double>()
        for (record in records) {
            val source = record.metadata.dataOrigin.packageName
            totals[source] = (totals[source] ?: 0.0) + value(record)
        }
        for ((source, total) in totals) {
            add(rows, name, total, unit, source)
        }
    }

    private fun <T : androidx.health.connect.client.records.Record> emitLatest(
        rows: JSONArray,
        name: String,
        unit: String,
        records: List<T>,
        value: (T) -> Any?,
    ) {
        val seen = linkedSetOf<String>()
        for (record in records) {
            val source = record.metadata.dataOrigin.packageName
            if (!seen.add(source)) continue
            add(rows, name, value(record), unit, source)
        }
    }

    private suspend fun <T : androidx.health.connect.client.records.Record> readLatest(
        client: HealthConnectClient,
        type: KClass<T>,
    ): List<T> = client.readRecords(
        ReadRecordsRequest(
            recordType = type,
            timeRangeFilter = TimeRangeFilter.before(Instant.now()),
            ascendingOrder = false,
            pageSize = 1000,
        ),
    ).records

    private suspend fun <T : androidx.health.connect.client.records.Record> readAll(
        client: HealthConnectClient,
        day: TimeRangeFilter,
        type: KClass<T>,
    ): List<T> = client.readRecords(
        ReadRecordsRequest(recordType = type, timeRangeFilter = day, pageSize = 1000),
    ).records

    private fun add(rows: JSONArray, name: String, value: Any?, unit: String, source: String) {
        if (value == null) return
        val text = when (value) {
            is Double -> if (value == 0.0) return else trimNumber(value)
            is Float -> if (value == 0f) return else trimNumber(value.toDouble())
            is Long -> if (value == 0L) return else value.toString()
            is Int -> if (value == 0) return else value.toString()
            else -> value.toString()
        }
        if (text.isEmpty() || text == "0") return
        rows.put(
            JSONObject()
                .put("name", name)
                .put("value", text)
                .put("unit", unit)
                .put("source", source),
        )
    }

    private fun trimNumber(value: Double): String {
        val rounded = kotlin.math.round(value * 10.0) / 10.0
        return if (rounded % 1.0 == 0.0) rounded.toLong().toString() else rounded.toString()
    }
}
