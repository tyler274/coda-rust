import Foundation
import UIKit
import UserNotifications

/// Binding for `duet-notify`. Screens stay in Rust.
///
/// Status is `not_determined`, `allowed`, or `denied`, matching the shipping
/// notification coordinator. A failed settings read leaves the pointer null.
/// The alert request asks for alerts only.
///
/// One insight alert is submitted only when new insight notifications are on
/// and alerts are allowed. Its title and body match the shipping coordinator.
/// Turning them off removes the pending daily request and leaves the planned
/// cycle id. A remote-notification token that arrives is kept in memory and
/// is not uploaded.
@_cdecl("duet_notify_status")
public func duetNotifyStatus() -> UnsafeMutablePointer<CChar>? {
    copyString(NotificationBridge.status())
}

@_cdecl("duet_notify_request")
public func duetNotifyRequest() {
    NotificationBridge.requestAccess()
}

@_cdecl("duet_notify_register_remote")
public func duetNotifyRegisterRemote() {
    NotificationBridge.registerRemote()
}

@_cdecl("duet_notify_last_planned")
public func duetNotifyLastPlanned() -> UnsafeMutablePointer<CChar>? {
    guard let cycle = NotificationBridge.lastPlanned() else { return nil }
    return copyString(cycle)
}

@_cdecl("duet_notify_schedule")
public func duetNotifySchedule(
    _ identifier: UnsafePointer<CChar>?,
    _ year: Int32,
    _ month: Int32,
    _ day: Int32,
    _ hour: Int32,
    _ minute: Int32,
    _ cycleID: UnsafePointer<CChar>?
) {
    guard let identifier, let cycleID else { return }
    NotificationBridge.schedule(
        identifier: String(cString: identifier),
        year: Int(year),
        month: Int(month),
        day: Int(day),
        hour: Int(hour),
        minute: Int(minute),
        cycleID: String(cString: cycleID)
    )
}

@_cdecl("duet_notify_remember_token")
public func duetNotifyRememberToken(_ token: UnsafePointer<CChar>?) {
    guard let token else { return }
    NotificationBridge.rememberToken(String(cString: token))
}

@_cdecl("duet_notify_insights_enabled")
public func duetNotifyInsightsEnabled() -> Int32 {
    NotificationBridge.insightsEnabled() ? 1 : 0
}

@_cdecl("duet_notify_set_insights")
public func duetNotifySetInsights(_ enabled: Int32) -> Int32 {
    NotificationBridge.setInsights(enabled != 0) ? 1 : 0
}

@_cdecl("duet_notify_cancel_insights")
public func duetNotifyCancelInsights() {
    NotificationBridge.cancelPending()
}

@_cdecl("duet_notify_open_settings")
public func duetNotifyOpenSettings() {
    NotificationBridge.openSettings()
}

@_cdecl("duet_notify_string_free")
public func duetNotifyStringFree(_ pointer: UnsafeMutablePointer<CChar>?) {
    free(pointer)
}

private func copyString(_ value: String) -> UnsafeMutablePointer<CChar>? {
    value.withCString { strdup($0) }
}

private enum NotificationBridge {
    private static let requestPrefix = "health.duet.notification.daily."
    private static let plannedKey = "coda.duet.insight.planned-cycle"
    private static let enabledKey = "coda.duet.insight.enabled"
    private static var remoteToken: String?

    static func status() -> String {
        let center = UNUserNotificationCenter.current()
        let ready = DispatchSemaphore(value: 0)
        let box = StatusBox()
        center.getNotificationSettings { settings in
            box.code = code(settings.authorizationStatus)
            ready.signal()
        }
        ready.wait()
        return box.code ?? "unreadable"
    }

    static func requestAccess() {
        let center = UNUserNotificationCenter.current()
        DispatchQueue.main.async {
            center.requestAuthorization(options: [.alert]) { _, _ in }
        }
    }

    private static func requestAccessAndWait() {
        let center = UNUserNotificationCenter.current()
        let ready = DispatchSemaphore(value: 0)
        let ask = {
            center.requestAuthorization(options: [.alert]) { _, _ in
                ready.signal()
            }
        }
        if Thread.isMainThread {
            ask()
        } else {
            DispatchQueue.main.async(execute: ask)
        }
        ready.wait()
    }

    static func registerRemote() {
        let register = {
            UIApplication.shared.registerForRemoteNotifications()
        }
        if Thread.isMainThread {
            register()
        } else {
            DispatchQueue.main.async(execute: register)
        }
    }

    static func rememberToken(_ token: String) {
        if remoteToken == token {
            return
        }
        remoteToken = token
    }

    static func lastPlanned() -> String? {
        UserDefaults.standard.string(forKey: plannedKey)
    }

    static func insightsEnabled() -> Bool {
        UserDefaults.standard.bool(forKey: enabledKey)
    }

    /// Saves the opt-in only when alerts are allowed. Turning it off drops
    /// pending daily ids and leaves the planned cycle id.
    static func setInsights(_ enabled: Bool) -> Bool {
        if !enabled {
            UserDefaults.standard.set(false, forKey: enabledKey)
            cancelPending()
            return false
        }
        if status() == "not_determined" {
            requestAccessAndWait()
        }
        guard status() == "allowed" else { return false }
        UserDefaults.standard.set(true, forKey: enabledKey)
        return true
    }

    static func cancelPending() {
        let center = UNUserNotificationCenter.current()
        center.getPendingNotificationRequests { requests in
            let pending = requests.map(\.identifier).filter { $0.hasPrefix(requestPrefix) }
            center.removePendingNotificationRequests(withIdentifiers: pending)
        }
    }

    static func openSettings() {
        guard let url = URL(string: UIApplication.openSettingsURLString) else { return }
        let open = { UIApplication.shared.open(url) }
        if Thread.isMainThread {
            open()
        } else {
            DispatchQueue.main.async(execute: open)
        }
    }

    /// Submits one insight alert and drops older daily ids. The opt-in and
    /// allowed alerts are both required. Denied or unread permission does not
    /// schedule and does not count as a planned cycle.
    static func schedule(
        identifier: String,
        year: Int,
        month: Int,
        day: Int,
        hour: Int,
        minute: Int,
        cycleID: String
    ) {
        guard status() == "allowed", insightsEnabled() else { return }
        registerRemote()
        let center = UNUserNotificationCenter.current()
        center.getPendingNotificationRequests { requests in
            let older = requests.map(\.identifier).filter {
                $0.hasPrefix(requestPrefix) && $0 != identifier
            }
            center.removePendingNotificationRequests(withIdentifiers: older)
            let content = UNMutableNotificationContent()
            content.threadIdentifier = "health.duet.insights"
            content.title = "New insights from Duet"
            content.body = "See what may be worth noticing in your health information."
            content.categoryIdentifier = "health.duet.notification.insight-ready"
            content.userInfo = ["duet_insight_cycle_id": cycleID]
            var when = DateComponents()
            when.year = year
            when.month = month
            when.day = day
            when.hour = hour
            when.minute = minute
            let request = UNNotificationRequest(
                identifier: identifier,
                content: content,
                trigger: UNCalendarNotificationTrigger(dateMatching: when, repeats: false)
            )
            center.add(request) { error in
                if error == nil {
                    UserDefaults.standard.set(cycleID, forKey: plannedKey)
                }
            }
        }
    }

    /// Provisional and ephemeral count as allowed, as they do in the shipping app.
    /// An unknown status is denied there; a missing read stays `unreadable`.
    private static func code(_ status: UNAuthorizationStatus) -> String {
        switch status {
        case .notDetermined:
            "not_determined"
        case .authorized, .provisional, .ephemeral:
            "allowed"
        case .denied:
            "denied"
        @unknown default:
            "denied"
        }
    }
}

private final class StatusBox: @unchecked Sendable {
    var code: String?
}
