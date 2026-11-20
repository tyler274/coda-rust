import Foundation
import UIKit
import UserNotifications

/// Binding for `duet-notify`. Screens stay in Rust.
///
/// Status is `not_determined`, `allowed`, or `denied`, matching the shipping
/// notification coordinator. A failed settings read leaves the pointer null.
/// The alert request asks for alerts only, and lock-screen text stays generic.
@_cdecl("duet_notify_status")
public func duetNotifyStatus() -> UnsafeMutablePointer<CChar>? {
    copyString(NotificationBridge.status())
}

@_cdecl("duet_notify_request")
public func duetNotifyRequest() {
    NotificationBridge.requestAccess()
}

@_cdecl("duet_notify_open_settings")
public func duetNotifyOpenSettings() {
    NotificationBridge.openSettings()
}

@_cdecl("duet_notify_push_token")
public func duetNotifyPushToken() -> UnsafeMutablePointer<CChar>? {
    guard let token = NotificationBridge.pushToken() else { return nil }
    return copyString(token)
}

@_cdecl("duet_notify_clear_on_sign_out")
public func duetNotifyClearOnSignOut() -> UnsafeMutablePointer<CChar>? {
    copyString(NotificationBridge.clearOnSignOut())
}

@_cdecl("duet_notify_string_free")
public func duetNotifyStringFree(_ pointer: UnsafeMutablePointer<CChar>?) {
    free(pointer)
}

private func copyString(_ value: String) -> UnsafeMutablePointer<CChar>? {
    value.withCString { strdup($0) }
}

public enum NotificationBridge {
    private static let tokenDefaultsKey = "coda.duet.notification.apns_token"

    /// Called by the app delegate when APNs issues or rotates a remote device token.
    public static func storeRemoteDeviceToken(_ tokenData: Data) {
        let hex = tokenData.map { String(format: "%02x", $0) }.joined()
        UserDefaults.standard.set(hex, forKey: tokenDefaultsKey)
    }

    static func pushToken() -> String? {
        guard let token = UserDefaults.standard.string(forKey: tokenDefaultsKey)?
            .trimmingCharacters(in: .whitespacesAndNewlines),
            !token.isEmpty
        else {
            return nil
        }
        return token
    }

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
            center.requestAuthorization(options: [.alert]) { granted, _ in
                if granted {
                    DispatchQueue.main.async {
                        UIApplication.shared.registerForRemoteNotifications()
                    }
                }
            }
        }
    }

    static func clearOnSignOut() -> String {
        let hadToken = pushToken() != nil
        UserDefaults.standard.removeObject(forKey: tokenDefaultsKey)
        let center = UNUserNotificationCenter.current()
        center.removeAllPendingNotificationRequests()
        center.removeAllDeliveredNotifications()
        DispatchQueue.main.async {
            UIApplication.shared.unregisterForRemoteNotifications()
        }
        return hadToken ? "cleared_all" : "cleared_alerts_only"
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
