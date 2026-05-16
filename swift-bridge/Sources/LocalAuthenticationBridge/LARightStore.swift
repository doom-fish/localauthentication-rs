import Foundation
import LocalAuthentication

@available(macOS 13.0, *)
final class LARightStoreHandle: LABridgeHandleBase {
    let value: LARightStore

    init(_ value: LARightStore) {
        self.value = value
    }
}

@available(macOS 13.0, *)
@inline(__always)
func laRightStore(_ ptr: UnsafeMutableRawPointer?) throws -> LARightStore {
    try laBorrowHandle(ptr, as: LARightStoreHandle.self, name: "LARightStore").value
}

@_cdecl("la_right_store_shared")
public func la_right_store_shared(
    _ outStore: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let outStore else {
        return laFail(LABridgeError.invalidArgument("missing output pointer for LARightStore"), errorOut)
    }

    if #available(macOS 13.0, *) {
        outStore.pointee = laRetainHandle(LARightStoreHandle(.shared))
        return LA_OK
    }

    return laFail(LABridgeError.bridgeFailed("LARightStore requires macOS 13.0"), errorOut)
}

@_cdecl("la_right_store_release")
public func la_right_store_release(_ ptr: UnsafeMutableRawPointer?) {
    laReleaseHandle(ptr)
}

@_cdecl("la_right_store_right_for_identifier")
public func la_right_store_right_for_identifier(
    _ storePtr: UnsafeMutableRawPointer?,
    _ identifier: UnsafePointer<CChar>?,
    _ outRight: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outRight else {
            throw LABridgeError.invalidArgument("missing output pointer for LAPersistedRight")
        }
        let identifier = try laRequiredString(identifier, name: "identifier")
        if #available(macOS 13.0, *) {
            let store = try laRightStore(storePtr)
            let right = try laAwait {
                try await store.right(forIdentifier: identifier)
            }
            outRight.pointee = laRetainHandle(LAPersistedRightHandle(right))
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LARightStore requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_right_store_save_right")
public func la_right_store_save_right(
    _ storePtr: UnsafeMutableRawPointer?,
    _ rightPtr: UnsafeMutableRawPointer?,
    _ identifier: UnsafePointer<CChar>?,
    _ outRight: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outRight else {
            throw LABridgeError.invalidArgument("missing output pointer for LAPersistedRight")
        }
        let identifier = try laRequiredString(identifier, name: "identifier")
        if #available(macOS 13.0, *) {
            let store = try laRightStore(storePtr)
            let right = try laRight(rightPtr)
            let persisted = try laAwait {
                try await store.saveRight(right, identifier: identifier)
            }
            outRight.pointee = laRetainHandle(LAPersistedRightHandle(persisted))
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LARightStore requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_right_store_save_right_with_secret")
public func la_right_store_save_right_with_secret(
    _ storePtr: UnsafeMutableRawPointer?,
    _ rightPtr: UnsafeMutableRawPointer?,
    _ identifier: UnsafePointer<CChar>?,
    _ secretBytes: UnsafePointer<UInt8>?,
    _ secretLen: UInt,
    _ outRight: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outRight else {
            throw LABridgeError.invalidArgument("missing output pointer for LAPersistedRight")
        }
        let identifier = try laRequiredString(identifier, name: "identifier")
        let secret = laData(secretBytes, len: Int(secretLen))
        if #available(macOS 13.0, *) {
            let store = try laRightStore(storePtr)
            let right = try laRight(rightPtr)
            let persisted = try laAwait {
                try await store.saveRight(right, identifier: identifier, secret: secret)
            }
            outRight.pointee = laRetainHandle(LAPersistedRightHandle(persisted))
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LARightStore requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_right_store_remove_right")
public func la_right_store_remove_right(
    _ storePtr: UnsafeMutableRawPointer?,
    _ rightPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        if #available(macOS 13.0, *) {
            let store = try laRightStore(storePtr)
            let right = try laPersistedRight(rightPtr)
            try laAwait {
                try await store.removeRight(right)
            }
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LARightStore requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_right_store_remove_right_for_identifier")
public func la_right_store_remove_right_for_identifier(
    _ storePtr: UnsafeMutableRawPointer?,
    _ identifier: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let identifier = try laRequiredString(identifier, name: "identifier")
        if #available(macOS 13.0, *) {
            let store = try laRightStore(storePtr)
            try laAwait {
                try await store.removeRight(forIdentifier: identifier)
            }
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LARightStore requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_right_store_remove_all_rights")
public func la_right_store_remove_all_rights(
    _ storePtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        if #available(macOS 13.0, *) {
            let store = try laRightStore(storePtr)
            try laAwait {
                try await store.removeAllRights()
            }
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LARightStore requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}
