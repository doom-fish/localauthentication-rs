import Foundation
import LocalAuthentication

let LA_CREDENTIAL_TYPE_APPLICATION_PASSWORD: Int32 = 0
let LA_CREDENTIAL_TYPE_SMART_CARD_PIN: Int32 = -3

@inline(__always)
func laCredentialType(_ raw: Int32) throws -> LACredentialType {
    switch raw {
    case LA_CREDENTIAL_TYPE_APPLICATION_PASSWORD:
        return .applicationPassword
    case LA_CREDENTIAL_TYPE_SMART_CARD_PIN:
        if #available(macOS 10.15.4, *) {
            return .smartCardPIN
        }
        throw LABridgeError.bridgeFailed("smart-card PIN credentials require macOS 10.15.4")
    default:
        throw LABridgeError.invalidArgument("unsupported credential type: \(raw)")
    }
}

@inline(__always)
func laCredentialData(
    _ bytes: UnsafePointer<UInt8>?,
    len: Int,
    hasCredential: Bool
) -> Data? {
    guard hasCredential else {
        return nil
    }
    return laData(bytes, len: len)
}
