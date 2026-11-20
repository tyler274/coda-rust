// Duet-only native bridge. Link this file and Security/UIKit
// into the iOS application target alongside HealthKitBridge.swift.
import Foundation
import Security
import UIKit

private enum RetentionFailure: Error { case unavailable }
private let retentionService = "coda.duet.protected-retention.v1"
private let retentionLock = NSLock()

private func retentionQuery(_ key: String) -> [String: Any] {
    [kSecClass as String: kSecClassGenericPassword,
     kSecAttrService as String: retentionService,
     kSecAttrAccount as String: key,
     kSecAttrSynchronizable as String: false]
}

// Keychain provides atomic replacement, encryption, device-only retention and locked-device denial.
// Drafts share the same protection as refresh credentials; no UserDefaults/backup/file fallback.
private func retentionCall(_ request: [String: Any]) throws -> Any {
    guard let operation = request["op"] as? String else { throw RetentionFailure.unavailable }
    // A nonsecret deny marker must remain writable while the Keychain is locked.
    // FileProtection.none is safe ONLY for this boolean; never store identity or draft data here.
    if operation == "cleanup_read" || operation == "cleanup_write" {
        retentionLock.lock(); defer { retentionLock.unlock() }
        let directory = try FileManager.default.url(for: .applicationSupportDirectory, in: .userDomainMask, appropriateFor: nil, create: true)
        var marker = directory.appendingPathComponent("duet-signout-pending")
        if operation == "cleanup_read" { return FileManager.default.fileExists(atPath: marker.path) }
        guard let pending = request["pending"] as? Bool else { throw RetentionFailure.unavailable }
        if pending {
            try Data([1]).write(to: marker, options: [.atomic, .noFileProtection])
            var values = URLResourceValues(); values.isExcludedFromBackup = true
            try marker.setResourceValues(values)
            let exports = FileManager.default.temporaryDirectory.appendingPathComponent("duet-exports")
            if FileManager.default.fileExists(atPath: exports.path) { try FileManager.default.removeItem(at: exports) }
        } else if FileManager.default.fileExists(atPath: marker.path) { try FileManager.default.removeItem(at: marker) }
        return NSNull()
    }
    if operation == "copy_link" {
        guard let value = request["value"] as? String else { throw RetentionFailure.unavailable }
        let copy = { UIPasteboard.general.setItems([["public.utf8-plain-text": value]], options: [.localOnly: true, .expirationDate: Date().addingTimeInterval(120)]) }
        if Thread.isMainThread { copy() } else { DispatchQueue.main.sync(execute: copy) }
        return NSNull()
    }
    if operation == "share_pdf" {
        guard let bytes = request["bytes"] as? [UInt8], bytes.count <= 20_000_000 else { throw RetentionFailure.unavailable }
        let present = { () -> Bool in
            guard let window = UIApplication.shared.connectedScenes.compactMap({ $0 as? UIWindowScene }).flatMap({ $0.windows }).first(where: { $0.isKeyWindow }),
                  var presenter = window.rootViewController else { return false }
            while let next = presenter.presentedViewController { presenter = next }
            do {
                let directory = FileManager.default.temporaryDirectory.appendingPathComponent("duet-exports", isDirectory: true)
                try FileManager.default.createDirectory(at: directory, withIntermediateDirectories: true, attributes: [.protectionKey: FileProtectionType.complete])
                // Previous interrupted shares are removed on the next export; no reusable public URI.
                for old in try FileManager.default.contentsOfDirectory(at: directory, includingPropertiesForKeys: nil) { try FileManager.default.removeItem(at: old) }
                let file = directory.appendingPathComponent(UUID().uuidString + ".pdf")
                try Data(bytes).write(to: file, options: [.atomic, .completeFileProtection])
                let sheet = UIActivityViewController(activityItems: [file], applicationActivities: nil)
                sheet.completionWithItemsHandler = { _, _, _, _ in try? FileManager.default.removeItem(at: file) }
                sheet.popoverPresentationController?.sourceView = presenter.view
                presenter.present(sheet, animated: true)
                return true
            } catch { return false }
        }
        guard Thread.isMainThread ? present() : DispatchQueue.main.sync(execute: present) else { throw RetentionFailure.unavailable }
        return NSNull()
    }
    if operation == "install_privacy_cover" {
        let install = { PrivacyCoverCoordinator.shared.install() }
        if Thread.isMainThread { install() } else { DispatchQueue.main.sync(execute: install) }
        return NSNull()
    }
    guard let key = request["key"] as? String, !key.isEmpty, key.utf8.count < 1024 else {
        throw RetentionFailure.unavailable
    }
    retentionLock.lock()
    defer { retentionLock.unlock() }
    var query = retentionQuery(key)
    switch operation {
    case "read":
        query[kSecReturnData as String] = true
        query[kSecMatchLimit as String] = kSecMatchLimitOne
        var result: CFTypeRef?
        let status = SecItemCopyMatching(query as CFDictionary, &result)
        if status == errSecItemNotFound { return NSNull() }
        guard status == errSecSuccess, let data = result as? Data else { throw RetentionFailure.unavailable }
        return Array(data)
    case "write":
        guard let numbers = request["bytes"] as? [UInt8], numbers.count <= 1_048_576 else { throw RetentionFailure.unavailable }
        let data = Data(numbers)
        let attributes: [String: Any] = [
            kSecValueData as String: data,
            kSecAttrAccessible as String: kSecAttrAccessibleWhenUnlockedThisDeviceOnly
        ]
        let status = SecItemUpdate(query as CFDictionary, attributes as CFDictionary)
        if status == errSecItemNotFound {
            query.merge(attributes) { _, value in value }
            guard SecItemAdd(query as CFDictionary, nil) == errSecSuccess else { throw RetentionFailure.unavailable }
        } else if status != errSecSuccess { throw RetentionFailure.unavailable }
        return NSNull()
    case "remove":
        let status = SecItemDelete(query as CFDictionary)
        guard status == errSecSuccess || status == errSecItemNotFound else { throw RetentionFailure.unavailable }
        return NSNull()
    default: throw RetentionFailure.unavailable
    }
}

@_cdecl("duet_secure_call")
public func duetSecureCall(_ pointer: UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>? {
    var reply: [String: Any] = ["ok": false]
    do {
        guard let pointer,
              let data = String(cString: pointer).data(using: .utf8),
              let request = try JSONSerialization.jsonObject(with: data) as? [String: Any]
        else { throw RetentionFailure.unavailable }
        reply = ["ok": true, "value": try retentionCall(request)]
    } catch { /* Never expose secrets or platform error payloads. */ }
    guard let data = try? JSONSerialization.data(withJSONObject: reply),
          let text = String(data: data, encoding: .utf8) else { return nil }
    return strdup(text)
}
@_cdecl("duet_secure_free")
public func duetSecureFree(_ pointer: UnsafeMutablePointer<CChar>?) { free(pointer) }

// Covers the Duet window whenever the app is inactive or backgrounded without touching Keychain state.
private final class PrivacyCoverCoordinator {
    static let shared = PrivacyCoverCoordinator()
    private var installed = false
    private var coverView: UIView?

    func install() {
        guard !installed else { return }
        installed = true
        let center = NotificationCenter.default
        center.addObserver(forName: UIApplication.willResignActiveNotification, object: nil, queue: .main) { [weak self] _ in
            self?.show()
        }
        center.addObserver(forName: UIApplication.didEnterBackgroundNotification, object: nil, queue: .main) { [weak self] _ in
            self?.show()
        }
        center.addObserver(forName: UIApplication.didBecomeActiveNotification, object: nil, queue: .main) { [weak self] _ in
            self?.hide()
        }
        center.addObserver(forName: UIApplication.willEnterForegroundNotification, object: nil, queue: .main) { [weak self] _ in
            self?.hide()
        }
    }

    private func show() {
        guard let window = UIApplication.shared.connectedScenes
            .compactMap({ $0 as? UIWindowScene })
            .flatMap({ $0.windows })
            .first(where: { $0.isKeyWindow }) else { return }
        let overlay = coverView ?? {
            let view = UIView(frame: window.bounds)
            view.autoresizingMask = [.flexibleWidth, .flexibleHeight]
            view.backgroundColor = UIColor.systemBackground
            view.isAccessibilityElement = true
            view.accessibilityLabel = "Duet protected screen"
            let label = UILabel()
            label.text = "Duet"
            label.font = UIFont.preferredFont(forTextStyle: .title2)
            label.textColor = UIColor.secondaryLabel
            label.translatesAutoresizingMaskIntoConstraints = false
            view.addSubview(label)
            NSLayoutConstraint.activate([
                label.centerXAnchor.constraint(equalTo: view.centerXAnchor),
                label.centerYAnchor.constraint(equalTo: view.centerYAnchor),
            ])
            coverView = view
            return view
        }()
        overlay.frame = window.bounds
        if overlay.superview !== window {
            window.addSubview(overlay)
        }
        window.bringSubviewToFront(overlay)
    }

    private func hide() {
        coverView?.removeFromSuperview()
    }
}
