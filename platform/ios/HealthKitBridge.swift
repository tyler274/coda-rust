import Foundation
import HealthKit

/// Binding for `duet-health`. Screens stay in Rust.
///
/// Status is `unavailable`, `permission`, or `connected`. A failed read leaves
/// the readings pointer null. Readings use the same `{granted, rows}` JSON as
/// Health Connect, with that bridge's sample names. HealthKit does not report
/// a denied read, so a completed request stays `connected` even when the store
/// is empty.
@_cdecl("duet_healthkit_status")
public func duetHealthKitStatus() -> UnsafeMutablePointer<CChar>? {
    copyString(HealthKitBridge.status())
}

@_cdecl("duet_healthkit_readings")
public func duetHealthKitReadings() -> UnsafeMutablePointer<CChar>? {
    guard let body = HealthKitBridge.readings() else { return nil }
    return copyString(body)
}

@_cdecl("duet_healthkit_request")
public func duetHealthKitRequest() {
    HealthKitBridge.requestAccess()
}

@_cdecl("duet_healthkit_string_free")
public func duetHealthKitStringFree(_ pointer: UnsafeMutablePointer<CChar>?) {
    free(pointer)
}

private func copyString(_ value: String) -> UnsafeMutablePointer<CChar>? {
    value.withCString { strdup($0) }
}

private enum HealthKitBridge {
    private static let requestedKey = "coda.duet.healthkit.requested"
    private static let store = HKHealthStore()
    private static let queue = DispatchQueue(label: "coda.duet.healthkit")

    static func status() -> String {
        guard HKHealthStore.isHealthDataAvailable() else { return "unavailable" }
        return UserDefaults.standard.bool(forKey: requestedKey) ? "connected" : "permission"
    }

    static func requestAccess() {
        guard HKHealthStore.isHealthDataAvailable() else { return }
        UserDefaults.standard.set(true, forKey: requestedKey)
        DispatchQueue.main.async {
            store.requestAuthorization(toShare: [], read: readTypes()) { _, _ in }
        }
    }

    /// Nil means the read itself failed. A type with no sample, or one query
    /// error, is left out so the rest of the store can still be shown.
    static func readings() -> String? {
        queue.sync { collect() }
    }

    private static func collect() -> String? {
        let start = Calendar.current.startOfDay(for: Date())
        let end = Date()
        var granted: [String] = []
        var rows: [[String: String]] = []
        if end > start {
            for sample in daySamples {
                guard let value = cumulative(sample, from: start, to: end),
                      let text = trimNumber(value) else { continue }
                granted.append(sample.name)
                rows.append(["name": sample.name, "value": text, "unit": sample.unitLabel])
            }
            if let minutes = sleepMinutes(from: start, to: end), let text = trimNumber(minutes) {
                granted.append("Sleep")
                rows.append(["name": "Sleep", "value": text, "unit": "min"])
            }
        }
        for sample in latestSamples {
            guard let value = latest(sample), let text = trimNumber(value * sample.scale) else { continue }
            granted.append(sample.name)
            rows.append(["name": sample.name, "value": text, "unit": sample.unitLabel])
        }
        if let pressure = bloodPressure() {
            granted.append("Blood pressure")
            rows.append(["name": "Blood pressure", "value": pressure, "unit": "mmHg"])
        }
        guard JSONSerialization.isValidJSONObject(["granted": granted, "rows": rows]),
              let data = try? JSONSerialization.data(withJSONObject: ["granted": granted, "rows": rows]),
              let text = String(data: data, encoding: .utf8) else {
            return nil
        }
        return text
    }

    private static func cumulative(_ sample: QuantitySample, from start: Date, to end: Date) -> Double? {
        guard let type = HKQuantityType.quantityType(forIdentifier: sample.identifier) else { return nil }
        let predicate = HKQuery.predicateForSamples(withStart: start, end: end, options: .strictStartDate)
        let result: Result<Double?, Error> = awaitQuery { done in
            let query = HKStatisticsQuery(
                quantityType: type,
                quantitySamplePredicate: predicate,
                options: .cumulativeSum
            ) { _, statistics, error in
                if let error {
                    done(.failure(error))
                    return
                }
                done(.success(statistics?.sumQuantity()?.doubleValue(for: sample.unit)))
            }
            store.execute(query)
        }
        guard case .success(let value) = result else { return nil }
        return value
    }

    private static func latest(_ sample: QuantitySample) -> Double? {
        guard let type = HKQuantityType.quantityType(forIdentifier: sample.identifier) else { return nil }
        let sort = NSSortDescriptor(key: HKSampleSortIdentifierEndDate, ascending: false)
        let result: Result<Double?, Error> = awaitQuery { done in
            let query = HKSampleQuery(sampleType: type, predicate: nil, limit: 1, sortDescriptors: [sort]) { _, samples, error in
                if let error {
                    done(.failure(error))
                    return
                }
                let quantity = (samples?.first as? HKQuantitySample)?.quantity.doubleValue(for: sample.unit)
                done(.success(quantity))
            }
            store.execute(query)
        }
        guard case .success(let value) = result else { return nil }
        return value
    }

    private static func sleepMinutes(from start: Date, to end: Date) -> Double? {
        guard let type = HKObjectType.categoryType(forIdentifier: .sleepAnalysis) else { return nil }
        let asleep: Set<Int> = [
            HKCategoryValueSleepAnalysis.asleepUnspecified.rawValue,
            HKCategoryValueSleepAnalysis.asleepCore.rawValue,
            HKCategoryValueSleepAnalysis.asleepDeep.rawValue,
            HKCategoryValueSleepAnalysis.asleepREM.rawValue,
        ]
        let predicate = HKQuery.predicateForSamples(withStart: start, end: end, options: .strictStartDate)
        let result: Result<[HKCategorySample], Error> = awaitQuery { done in
            let query = HKSampleQuery(sampleType: type, predicate: predicate, limit: HKObjectQueryNoLimit, sortDescriptors: nil) { _, samples, error in
                if let error {
                    done(.failure(error))
                    return
                }
                done(.success((samples as? [HKCategorySample]) ?? []))
            }
            store.execute(query)
        }
        guard case .success(let samples) = result else { return nil }
        let minutes = samples.reduce(0.0) { total, sample in
            guard asleep.contains(sample.value) else { return total }
            return total + sample.endDate.timeIntervalSince(sample.startDate) / 60
        }
        return minutes
    }

    private static func bloodPressure() -> String? {
        guard let correlation = HKCorrelationType.correlationType(forIdentifier: .bloodPressure),
              let systolic = HKQuantityType.quantityType(forIdentifier: .bloodPressureSystolic),
              let diastolic = HKQuantityType.quantityType(forIdentifier: .bloodPressureDiastolic) else {
            return nil
        }
        let sort = NSSortDescriptor(key: HKSampleSortIdentifierEndDate, ascending: false)
        let result: Result<HKCorrelation?, Error> = awaitQuery { done in
            let query = HKSampleQuery(sampleType: correlation, predicate: nil, limit: 1, sortDescriptors: [sort]) { _, samples, error in
                if let error {
                    done(.failure(error))
                    return
                }
                done(.success(samples?.first as? HKCorrelation))
            }
            store.execute(query)
        }
        guard case .success(let item) = result, let item else { return nil }
        let high = (item.objects(for: systolic).first as? HKQuantitySample)?
            .quantity.doubleValue(for: .millimeterOfMercury())
        let low = (item.objects(for: diastolic).first as? HKQuantitySample)?
            .quantity.doubleValue(for: .millimeterOfMercury())
        guard let high, let low, high > 0, low > 0 else { return nil }
        return "\(Int(high.rounded()))/\(Int(low.rounded()))"
    }

    private static func awaitQuery<T>(_ start: (@escaping (Result<T, Error>) -> Void) -> Void) -> Result<T, Error> {
        let box = QueryBox<T>()
        let ready = DispatchSemaphore(value: 0)
        start { result in
            box.result = result
            ready.signal()
        }
        ready.wait()
        return box.result ?? .failure(ReadFailure())
    }

    private static func readTypes() -> Set<HKObjectType> {
        var types = Set<HKObjectType>()
        for sample in daySamples + latestSamples {
            if let type = HKQuantityType.quantityType(forIdentifier: sample.identifier) {
                types.insert(type)
            }
        }
        if let sleep = HKObjectType.categoryType(forIdentifier: .sleepAnalysis) {
            types.insert(sleep)
        }
        if let pressure = HKCorrelationType.correlationType(forIdentifier: .bloodPressure) {
            types.insert(pressure)
        }
        if let systolic = HKQuantityType.quantityType(forIdentifier: .bloodPressureSystolic) {
            types.insert(systolic)
        }
        if let diastolic = HKQuantityType.quantityType(forIdentifier: .bloodPressureDiastolic) {
            types.insert(diastolic)
        }
        return types
    }

    private static func trimNumber(_ value: Double) -> String? {
        guard value.isFinite, value != 0 else { return nil }
        let rounded = (value * 10).rounded() / 10
        guard rounded != 0 else { return nil }
        if rounded.rounded() == rounded {
            return String(Int(rounded))
        }
        return String(rounded)
    }
}

private struct QuantitySample {
    let name: String
    let identifier: HKQuantityTypeIdentifier
    let unit: HKUnit
    let unitLabel: String
    var scale: Double = 1
}

private let daySamples: [QuantitySample] = [
    QuantitySample(name: "Steps", identifier: .stepCount, unit: .count(), unitLabel: "steps"),
    QuantitySample(name: "Distance", identifier: .distanceWalkingRunning, unit: .meter(), unitLabel: "m"),
    QuantitySample(name: "Active energy", identifier: .activeEnergyBurned, unit: .kilocalorie(), unitLabel: "kcal"),
    QuantitySample(name: "Water", identifier: .dietaryWater, unit: .liter(), unitLabel: "L"),
    QuantitySample(name: "Dietary energy", identifier: .dietaryEnergyConsumed, unit: .kilocalorie(), unitLabel: "kcal"),
    QuantitySample(name: "Exercise", identifier: .appleExerciseTime, unit: .minute(), unitLabel: "min"),
]

private let latestSamples: [QuantitySample] = [
    QuantitySample(name: "Resting heart rate", identifier: .restingHeartRate, unit: .count().unitDivided(by: .minute()), unitLabel: "bpm"),
    QuantitySample(name: "Weight", identifier: .bodyMass, unit: .gramUnit(with: .kilo), unitLabel: "kg"),
    QuantitySample(name: "Height", identifier: .height, unit: .meter(), unitLabel: "m"),
    QuantitySample(name: "Body fat", identifier: .bodyFatPercentage, unit: .percent(), unitLabel: "%", scale: 100),
    QuantitySample(name: "Lean body mass", identifier: .leanBodyMass, unit: .gramUnit(with: .kilo), unitLabel: "kg"),
    QuantitySample(
        name: "Blood glucose",
        identifier: .bloodGlucose,
        unit: .gramUnit(with: .milli).unitDivided(by: .literUnit(with: .deci)),
        unitLabel: "mg/dL"
    ),
    QuantitySample(name: "Body temperature", identifier: .bodyTemperature, unit: .degreeCelsius(), unitLabel: "C"),
    QuantitySample(name: "Basal body temperature", identifier: .basalBodyTemperature, unit: .degreeCelsius(), unitLabel: "C"),
    QuantitySample(name: "Oxygen saturation", identifier: .oxygenSaturation, unit: .percent(), unitLabel: "%", scale: 100),
    QuantitySample(
        name: "VO2 max",
        identifier: .vo2Max,
        unit: .literUnit(with: .milli).unitDivided(by: .gramUnit(with: .kilo).unitMultiplied(by: .minute())),
        unitLabel: "mL/kg/min"
    ),
]

private final class QueryBox<T>: @unchecked Sendable {
    var result: Result<T, Error>?
}

private struct ReadFailure: Error {}
