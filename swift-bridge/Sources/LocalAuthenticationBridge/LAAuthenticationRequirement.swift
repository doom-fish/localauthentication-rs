import Foundation
import LocalAuthentication

final class LAAuthenticationRequirementHandle: LABridgeHandleBase {
    let value: LAAuthenticationRequirement

    init(_ value: LAAuthenticationRequirement) {
        self.value = value
    }
}

final class LABiometryFallbackRequirementHandle: LABridgeHandleBase {
    let value: LABiometryFallbackRequirement

    init(_ value: LABiometryFallbackRequirement) {
        self.value = value
    }
}

@inline(__always)
func laAuthenticationRequirement(_ ptr: UnsafeMutableRawPointer?) throws -> LAAuthenticationRequirement {
    try laBorrowHandle(ptr, as: LAAuthenticationRequirementHandle.self, name: "LAAuthenticationRequirement").value
}

@inline(__always)
func laBiometryFallbackRequirement(_ ptr: UnsafeMutableRawPointer?) throws -> LABiometryFallbackRequirement {
    try laBorrowHandle(ptr, as: LABiometryFallbackRequirementHandle.self, name: "LABiometryFallbackRequirement").value
}

@_cdecl("la_authentication_requirement_default")
public func la_authentication_requirement_default(
    _ outRequirement: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let outRequirement else {
        return laFail(LABridgeError.invalidArgument("missing output pointer for LAAuthenticationRequirement"), errorOut)
    }

    if #available(macOS 13.0, *) {
        outRequirement.pointee = laRetainHandle(LAAuthenticationRequirementHandle(.default))
        return LA_OK
    }

    return laFail(LABridgeError.bridgeFailed("LAAuthenticationRequirement requires macOS 13.0"), errorOut)
}

@_cdecl("la_authentication_requirement_biometry")
public func la_authentication_requirement_biometry(
    _ outRequirement: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let outRequirement else {
        return laFail(LABridgeError.invalidArgument("missing output pointer for LAAuthenticationRequirement"), errorOut)
    }

    if #available(macOS 13.0, *) {
        outRequirement.pointee = laRetainHandle(LAAuthenticationRequirementHandle(.biometry))
        return LA_OK
    }

    return laFail(LABridgeError.bridgeFailed("LAAuthenticationRequirement requires macOS 13.0"), errorOut)
}

@_cdecl("la_authentication_requirement_biometry_current_set")
public func la_authentication_requirement_biometry_current_set(
    _ outRequirement: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let outRequirement else {
        return laFail(LABridgeError.invalidArgument("missing output pointer for LAAuthenticationRequirement"), errorOut)
    }

    if #available(macOS 13.0, *) {
        outRequirement.pointee = laRetainHandle(LAAuthenticationRequirementHandle(.biometryCurrentSet))
        return LA_OK
    }

    return laFail(LABridgeError.bridgeFailed("LAAuthenticationRequirement requires macOS 13.0"), errorOut)
}

@_cdecl("la_authentication_requirement_biometry_with_fallback")
public func la_authentication_requirement_biometry_with_fallback(
    _ fallbackPtr: UnsafeMutableRawPointer?,
    _ outRequirement: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outRequirement else {
            throw LABridgeError.invalidArgument("missing output pointer for LAAuthenticationRequirement")
        }
        if #available(macOS 13.0, *) {
            let fallback = try laBiometryFallbackRequirement(fallbackPtr)
            outRequirement.pointee = laRetainHandle(
                LAAuthenticationRequirementHandle(.biometry(fallback: fallback))
            )
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAAuthenticationRequirement requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_authentication_requirement_release")
public func la_authentication_requirement_release(_ ptr: UnsafeMutableRawPointer?) {
    laReleaseHandle(ptr)
}

@_cdecl("la_biometry_fallback_requirement_default")
public func la_biometry_fallback_requirement_default(
    _ outRequirement: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let outRequirement else {
        return laFail(LABridgeError.invalidArgument("missing output pointer for LABiometryFallbackRequirement"), errorOut)
    }

    if #available(macOS 13.0, *) {
        outRequirement.pointee = laRetainHandle(LABiometryFallbackRequirementHandle(.default))
        return LA_OK
    }

    return laFail(LABridgeError.bridgeFailed("LABiometryFallbackRequirement requires macOS 13.0"), errorOut)
}

@_cdecl("la_biometry_fallback_requirement_device_passcode")
public func la_biometry_fallback_requirement_device_passcode(
    _ outRequirement: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let outRequirement else {
        return laFail(LABridgeError.invalidArgument("missing output pointer for LABiometryFallbackRequirement"), errorOut)
    }

    if #available(macOS 13.0, *) {
        outRequirement.pointee = laRetainHandle(LABiometryFallbackRequirementHandle(.devicePasscode))
        return LA_OK
    }

    return laFail(LABridgeError.bridgeFailed("LABiometryFallbackRequirement requires macOS 13.0"), errorOut)
}

@_cdecl("la_biometry_fallback_requirement_release")
public func la_biometry_fallback_requirement_release(_ ptr: UnsafeMutableRawPointer?) {
    laReleaseHandle(ptr)
}
