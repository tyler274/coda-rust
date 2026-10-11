import Foundation
import UserNotifications

/// Binding for `duet-notify`. Screens stay in Rust.
///
/// Status is `not_determined`, `allowed`, or `denied`, matching the shipping
/// notification coordinator. A failed settings read leaves the pointer null.
/// The request asks for alerts only. It does not schedule insights or register
/// a push token.
@_cdecl("duet_notify_status")
public func duetNotifyStatus() -> UnsafeMutablePointer<CChar>? {
    copyString(NotificationBridge.status())
}

@_cdecl("duet_notify_request")
public func duetNotifyRequest() {
    NotificationBridge.requestAccess()
}

@_cdecl("duet_notify_string_free")
public func duetNotifyStringFree(_ pointer: UnsafeMutablePointer<CChar>?) {
    free(pointer)
}

private func copyString(_ value: String) -> UnsafeMutablePointer<CChar>? {
    value.withCString { strdup($0) }
}

private enum NotificationBridge {
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
