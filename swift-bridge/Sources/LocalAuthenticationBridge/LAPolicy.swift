import Foundation
import LocalAuthentication

let LA_POLICY_DEVICE_OWNER_AUTHENTICATION_WITH_BIOMETRICS: Int32 = 1
let LA_POLICY_DEVICE_OWNER_AUTHENTICATION: Int32 = 2
let LA_POLICY_DEVICE_OWNER_AUTHENTICATION_WITH_WATCH: Int32 = 3
let LA_POLICY_DEVICE_OWNER_AUTHENTICATION_WITH_BIOMETRICS_OR_WATCH: Int32 = 4
let LA_POLICY_DEVICE_OWNER_AUTHENTICATION_WITH_COMPANION: Int32 = 3
let LA_POLICY_DEVICE_OWNER_AUTHENTICATION_WITH_BIOMETRICS_OR_COMPANION: Int32 = 4
let LA_POLICY_DEVICE_OWNER_AUTHENTICATION_WITH_WRIST_DETECTION: Int32 = 5

@inline(__always)
func laPolicy(_ raw: Int32) throws -> LAPolicy {
    switch raw {
    case LA_POLICY_DEVICE_OWNER_AUTHENTICATION_WITH_BIOMETRICS:
        return .deviceOwnerAuthenticationWithBiometrics
    case LA_POLICY_DEVICE_OWNER_AUTHENTICATION:
        return .deviceOwnerAuthentication
    case LA_POLICY_DEVICE_OWNER_AUTHENTICATION_WITH_COMPANION,
        LA_POLICY_DEVICE_OWNER_AUTHENTICATION_WITH_WATCH:
        if #available(macOS 15.0, *) {
            return .deviceOwnerAuthenticationWithCompanion
        }
        if #available(macOS 10.15, *) {
            return .deviceOwnerAuthenticationWithWatch
        }
        throw LABridgeError.bridgeFailed("watch authentication requires macOS 10.15")
    case LA_POLICY_DEVICE_OWNER_AUTHENTICATION_WITH_BIOMETRICS_OR_COMPANION,
        LA_POLICY_DEVICE_OWNER_AUTHENTICATION_WITH_BIOMETRICS_OR_WATCH:
        if #available(macOS 15.0, *) {
            return .deviceOwnerAuthenticationWithBiometricsOrCompanion
        }
        if #available(macOS 10.15, *) {
            return .deviceOwnerAuthenticationWithBiometricsOrWatch
        }
        throw LABridgeError.bridgeFailed("watch authentication requires macOS 10.15")
    case LA_POLICY_DEVICE_OWNER_AUTHENTICATION_WITH_WRIST_DETECTION:
        throw LABridgeError.bridgeFailed("wrist-detection authentication is not available on macOS")
    default:
        throw LABridgeError.invalidArgument("unsupported policy value: \(raw)")
    }
}
