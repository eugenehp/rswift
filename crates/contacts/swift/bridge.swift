import Foundation
import Contacts

// ═══════════════════════════════════════════════════════════════════════════
// Contacts — address book access
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("contacts_available")
public func contactsAvailable() -> Bool { true }

@_cdecl("contacts_authorization_status")
public func contactsAuthorizationStatus() -> Int {
    CNContactStore.authorizationStatus(for: .contacts).rawValue
}

@_cdecl("contacts_request_access")
public func contactsRequestAccess(
    _ cb: @convention(c) (Bool, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) {
    let store = CNContactStore()
    store.requestAccess(for: .contacts) { granted, _ in
        cb(granted, ud)
    }
}

/// Fetch contacts as JSON lines: {"given":"John","family":"Doe","email":"j@d.com","phone":"+1..."}
@_cdecl("contacts_fetch_all")
public func contactsFetchAll(_ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    let store = CNContactStore()
    let keys: [CNKeyDescriptor] = [
        CNContactGivenNameKey as CNKeyDescriptor,
        CNContactFamilyNameKey as CNKeyDescriptor,
        CNContactEmailAddressesKey as CNKeyDescriptor,
        CNContactPhoneNumbersKey as CNKeyDescriptor,
        CNContactOrganizationNameKey as CNKeyDescriptor,
    ]
    let request = CNContactFetchRequest(keysToFetch: keys)

    var lines: [String] = []
    do {
        try store.enumerateContacts(with: request) { contact, _ in
            let email = contact.emailAddresses.first?.value as String? ?? ""
            let phone = contact.phoneNumbers.first?.value.stringValue ?? ""
            // Simple JSON-ish line
            let line = "{\"given\":\"\(esc(contact.givenName))\",\"family\":\"\(esc(contact.familyName))\",\"org\":\"\(esc(contact.organizationName))\",\"email\":\"\(esc(email))\",\"phone\":\"\(esc(phone))\"}"
            lines.append(line)
        }
    } catch {
        return -1
    }

    let result = lines.joined(separator: "\n")
    let data = Array(result.utf8)
    let len = min(data.count, bufLen)
    for i in 0..<len { buf[i] = data[i] }
    return data.count // return full length even if truncated
}

@_cdecl("contacts_count")
public func contactsCount() -> Int {
    let store = CNContactStore()
    let keys: [CNKeyDescriptor] = [CNContactGivenNameKey as CNKeyDescriptor]
    let request = CNContactFetchRequest(keysToFetch: keys)
    var count = 0
    do {
        try store.enumerateContacts(with: request) { _, _ in count += 1 }
    } catch {
        return -1
    }
    return count
}

private func esc(_ s: String) -> String {
    s.replacingOccurrences(of: "\\", with: "\\\\")
     .replacingOccurrences(of: "\"", with: "\\\"")
     .replacingOccurrences(of: "\n", with: "\\n")
}
