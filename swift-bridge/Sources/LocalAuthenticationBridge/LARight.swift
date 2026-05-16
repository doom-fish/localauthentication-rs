import Foundation
import LocalAuthentication

@available(macOS 13.0, *)
final class LARightHandle: LABridgeHandleBase {
    let value: LARight

    init(_ value: LARight) {
        self.value = value
    }
}

@available(macOS 13.0, *)
@inline(__always)
func laRight(_ ptr: UnsafeMutableRawPointer?) throws -> LARight {
    try laBorrowHandle(ptr, as: LARightHandle.self, name: "LARight").value
}

@_cdecl("la_right_new")
public func la_right_new(
    _ outRight: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let outRight else {
        return laFail(LABridgeError.invalidArgument("missing output pointer for LARight"), errorOut)
    }

    if #available(macOS 13.0, *) {
        outRight.pointee = laRetainHandle(LARightHandle(LARight()))
        return LA_OK
    }

    return laFail(LABridgeError.bridgeFailed("LARight requires macOS 13.0"), errorOut)
}

@_cdecl("la_right_new_with_requirement")
public func la_right_new_with_requirement(
    _ requirementPtr: UnsafeMutableRawPointer?,
    _ outRight: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outRight else {
            throw LABridgeError.invalidArgument("missing output pointer for LARight")
        }
        if #available(macOS 13.0, *) {
            let requirement = try laAuthenticationRequirement(requirementPtr)
            outRight.pointee = laRetainHandle(LARightHandle(LARight(requirement: requirement)))
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LARight requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_right_release")
public func la_right_release(_ ptr: UnsafeMutableRawPointer?) {
    laReleaseHandle(ptr)
}

@_cdecl("la_right_get_state")
public func la_right_get_state(
    _ rightPtr: UnsafeMutableRawPointer?,
    _ outState: UnsafeMutablePointer<Int32>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outState else {
            throw LABridgeError.invalidArgument("missing output pointer for LARight.state")
        }
        if #available(macOS 13.0, *) {
            let right = try laRight(rightPtr)
            outState.pointee = Int32(right.state.rawValue)
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LARight requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_right_get_tag")
public func la_right_get_tag(
    _ rightPtr: UnsafeMutableRawPointer?,
    _ outTag: UnsafeMutablePointer<Int64>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outTag else {
            throw LABridgeError.invalidArgument("missing output pointer for LARight.tag")
        }
        if #available(macOS 13.0, *) {
            let right = try laRight(rightPtr)
            outTag.pointee = Int64(right.tag)
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LARight requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_right_set_tag")
public func la_right_set_tag(
    _ rightPtr: UnsafeMutableRawPointer?,
    _ tag: Int64,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        if #available(macOS 13.0, *) {
            let right = try laRight(rightPtr)
            right.tag = Int(tag)
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LARight requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_right_authorize")
public func la_right_authorize(
    _ rightPtr: UnsafeMutableRawPointer?,
    _ localizedReason: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let reason = try laRequiredString(localizedReason, name: "localized reason")
        if #available(macOS 13.0, *) {
            let right = try laRight(rightPtr)
            try laAwait {
                try await right.authorize(localizedReason: reason)
            }
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LARight requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_right_check_can_authorize")
public func la_right_check_can_authorize(
    _ rightPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        if #available(macOS 13.0, *) {
            let right = try laRight(rightPtr)
            try laAwait {
                try await right.checkCanAuthorize()
            }
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LARight requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_right_deauthorize")
public func la_right_deauthorize(
    _ rightPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        if #available(macOS 13.0, *) {
            let right = try laRight(rightPtr)
            try laAwait {
                await right.deauthorize()
                return ()
            }
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LARight requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}
