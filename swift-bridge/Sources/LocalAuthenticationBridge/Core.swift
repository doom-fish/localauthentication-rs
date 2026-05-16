import Foundation
import LocalAuthentication

let LA_OK: Int32 = 0
let LA_INVALID_ARGUMENT: Int32 = -10_000
let LA_TIMED_OUT: Int32 = -10_001
let LA_BRIDGE_FAILED: Int32 = -10_002
let LA_UNKNOWN: Int32 = -10_099

let LA_POLICY_DEVICE_OWNER_AUTHENTICATION_WITH_BIOMETRICS: Int32 = 1
let LA_POLICY_DEVICE_OWNER_AUTHENTICATION: Int32 = 2
let LA_POLICY_DEVICE_OWNER_AUTHENTICATION_WITH_COMPANION: Int32 = 3
let LA_POLICY_DEVICE_OWNER_AUTHENTICATION_WITH_BIOMETRICS_OR_COMPANION: Int32 = 4

let LA_BIOMETRY_NONE: Int32 = 0
let LA_BIOMETRY_TOUCH_ID: Int32 = 1
let LA_BIOMETRY_FACE_ID: Int32 = 2
let LA_BIOMETRY_OPTIC_ID: Int32 = 4

enum LABridgeError: LocalizedError {
    case invalidArgument(String)
    case timedOut(String)
    case bridgeFailed(String)
    case unknown(String)

    var errorDescription: String? {
        switch self {
        case .invalidArgument(let message):
            return message
        case .timedOut(let message):
            return message
        case .bridgeFailed(let message):
            return message
        case .unknown(let message):
            return message
        }
    }

    var statusCode: Int32 {
        switch self {
        case .invalidArgument:
            return LA_INVALID_ARGUMENT
        case .timedOut:
            return LA_TIMED_OUT
        case .bridgeFailed:
            return LA_BRIDGE_FAILED
        case .unknown:
            return LA_UNKNOWN
        }
    }
}

@inline(__always)
func laCString(_ string: String) -> UnsafeMutablePointer<CChar>? {
    string.withCString { strdup($0) }
}

@inline(__always)
func laWriteError(
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ message: String
) {
    errorOut?.pointee = laCString(message)
}

@inline(__always)
func laFail(
    _ error: LABridgeError,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    laWriteError(errorOut, error.localizedDescription)
    return error.statusCode
}

@inline(__always)
func laRetain<T: AnyObject>(_ object: T) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
func laBorrow<T: AnyObject>(_ ptr: UnsafeMutableRawPointer) -> T {
    Unmanaged<T>.fromOpaque(ptr).takeUnretainedValue()
}

@_cdecl("la_context_release")
public func la_context_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AnyObject>.fromOpaque(ptr).release()
}

@inline(__always)
func laContext(_ ptr: UnsafeMutableRawPointer?) throws -> LAContext {
    guard let ptr else {
        throw LABridgeError.invalidArgument("missing `LAContext` handle")
    }
    return laBorrow(ptr)
}

@inline(__always)
func laPolicy(_ raw: Int32) throws -> LAPolicy {
    switch raw {
    case LA_POLICY_DEVICE_OWNER_AUTHENTICATION_WITH_BIOMETRICS:
        return .deviceOwnerAuthenticationWithBiometrics
    case LA_POLICY_DEVICE_OWNER_AUTHENTICATION:
        return .deviceOwnerAuthentication
    case LA_POLICY_DEVICE_OWNER_AUTHENTICATION_WITH_COMPANION:
        if #available(macOS 15.0, *) {
            return .deviceOwnerAuthenticationWithCompanion
        }
        throw LABridgeError.bridgeFailed(
            "companion-device authentication requires macOS 15.0"
        )
    case LA_POLICY_DEVICE_OWNER_AUTHENTICATION_WITH_BIOMETRICS_OR_COMPANION:
        if #available(macOS 15.0, *) {
            return .deviceOwnerAuthenticationWithBiometricsOrCompanion
        }
        throw LABridgeError.bridgeFailed(
            "biometric-or-companion authentication requires macOS 15.0"
        )
    default:
        throw LABridgeError.invalidArgument("unsupported policy value: \(raw)")
    }
}

@inline(__always)
func laOptionalString(_ value: UnsafePointer<CChar>?) -> String? {
    guard let value else { return nil }
    return String(cString: value)
}

@inline(__always)
func laWriteFrameworkError(
    _ error: NSError?,
    _ outCode: UnsafeMutablePointer<Int32>?,
    _ outMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    outCode?.pointee = Int32(error?.code ?? 0)
    outMessage?.pointee = error.flatMap { laCString($0.localizedDescription) }
}

@inline(__always)
func laCopyData(
    _ data: Data,
    _ outBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>?,
    _ outLen: UnsafeMutablePointer<UInt>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let outBytes, let outLen else {
        return laFail(
            .invalidArgument("missing output pointers for byte buffer"),
            errorOut
        )
    }

    outLen.pointee = UInt(data.count)
    if data.isEmpty {
        outBytes.pointee = nil
        return LA_OK
    }

    guard let raw = malloc(data.count) else {
        return laFail(.bridgeFailed("malloc failed"), errorOut)
    }
    let buffer = raw.assumingMemoryBound(to: UInt8.self)
    data.copyBytes(to: buffer, count: data.count)
    outBytes.pointee = buffer
    return LA_OK
}

@inline(__always)
func laBiometryTypeValue(for context: LAContext) -> Int32 {
    guard #available(macOS 10.13.2, *) else {
        return LA_BIOMETRY_NONE
    }

    let biometryType = context.biometryType
    if biometryType == .none {
        return LA_BIOMETRY_NONE
    }
    if biometryType == .touchID {
        return LA_BIOMETRY_TOUCH_ID
    }
    if #available(macOS 10.15, *), biometryType == .faceID {
        return LA_BIOMETRY_FACE_ID
    }
    if #available(macOS 14.0, *), biometryType == .opticID {
        return LA_BIOMETRY_OPTIC_ID
    }
    return LA_BIOMETRY_NONE
}
