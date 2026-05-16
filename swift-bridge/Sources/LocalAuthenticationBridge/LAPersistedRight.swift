import Foundation
import LocalAuthentication
import Security

@available(macOS 13.0, *)
final class LAPersistedRightHandle: LABridgeHandleBase {
    let value: LAPersistedRight

    init(_ value: LAPersistedRight) {
        self.value = value
    }
}

@available(macOS 13.0, *)
final class LASecretHandle: LABridgeHandleBase {
    let value: LASecret

    init(_ value: LASecret) {
        self.value = value
    }
}

@available(macOS 13.0, *)
final class LAPrivateKeyHandle: LABridgeHandleBase {
    let value: LAPrivateKey

    init(_ value: LAPrivateKey) {
        self.value = value
    }
}

@available(macOS 13.0, *)
@inline(__always)
func laPersistedRight(_ ptr: UnsafeMutableRawPointer?) throws -> LAPersistedRight {
    try laBorrowHandle(ptr, as: LAPersistedRightHandle.self, name: "LAPersistedRight").value
}

@available(macOS 13.0, *)
@inline(__always)
func laSecret(_ ptr: UnsafeMutableRawPointer?) throws -> LASecret {
    try laBorrowHandle(ptr, as: LASecretHandle.self, name: "LASecret").value
}

@available(macOS 13.0, *)
@inline(__always)
func laPrivateKey(_ ptr: UnsafeMutableRawPointer?) throws -> LAPrivateKey {
    try laBorrowHandle(ptr, as: LAPrivateKeyHandle.self, name: "LAPrivateKey").value
}

@available(macOS 13.0, *)
@inline(__always)
func laSecKeyExchangeParameters(
    requestedSize: Int64,
    sharedInfo: UnsafePointer<UInt8>?,
    sharedInfoLen: UInt,
    hasSharedInfo: UInt8
) throws -> [SecKeyKeyExchangeParameter: Any] {
    var parameters: [SecKeyKeyExchangeParameter: Any] = [:]

    if requestedSize >= 0 {
        parameters[.requestedSize] = Int(requestedSize)
    }
    if hasSharedInfo != 0 {
        parameters[.sharedInfo] = laData(sharedInfo, len: Int(sharedInfoLen))
    }

    return parameters
}

@_cdecl("la_persisted_right_release")
public func la_persisted_right_release(_ ptr: UnsafeMutableRawPointer?) {
    laReleaseHandle(ptr)
}

@_cdecl("la_persisted_right_get_state")
public func la_persisted_right_get_state(
    _ rightPtr: UnsafeMutableRawPointer?,
    _ outState: UnsafeMutablePointer<Int32>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outState else {
            throw LABridgeError.invalidArgument("missing output pointer for LAPersistedRight.state")
        }
        if #available(macOS 13.0, *) {
            let right = try laPersistedRight(rightPtr)
            outState.pointee = Int32(right.state.rawValue)
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAPersistedRight requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_persisted_right_get_tag")
public func la_persisted_right_get_tag(
    _ rightPtr: UnsafeMutableRawPointer?,
    _ outTag: UnsafeMutablePointer<Int64>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outTag else {
            throw LABridgeError.invalidArgument("missing output pointer for LAPersistedRight.tag")
        }
        if #available(macOS 13.0, *) {
            let right = try laPersistedRight(rightPtr)
            outTag.pointee = Int64(right.tag)
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAPersistedRight requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_persisted_right_set_tag")
public func la_persisted_right_set_tag(
    _ rightPtr: UnsafeMutableRawPointer?,
    _ tag: Int64,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        if #available(macOS 13.0, *) {
            let right = try laPersistedRight(rightPtr)
            right.tag = Int(tag)
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAPersistedRight requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_persisted_right_authorize")
public func la_persisted_right_authorize(
    _ rightPtr: UnsafeMutableRawPointer?,
    _ localizedReason: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let reason = try laRequiredString(localizedReason, name: "localized reason")
        if #available(macOS 13.0, *) {
            let right = try laPersistedRight(rightPtr)
            try laAwait {
                try await right.authorize(localizedReason: reason)
            }
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAPersistedRight requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_persisted_right_check_can_authorize")
public func la_persisted_right_check_can_authorize(
    _ rightPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        if #available(macOS 13.0, *) {
            let right = try laPersistedRight(rightPtr)
            try laAwait {
                try await right.checkCanAuthorize()
            }
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAPersistedRight requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_persisted_right_deauthorize")
public func la_persisted_right_deauthorize(
    _ rightPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        if #available(macOS 13.0, *) {
            let right = try laPersistedRight(rightPtr)
            try laAwait {
                await right.deauthorize()
                return ()
            }
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAPersistedRight requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_persisted_right_get_key")
public func la_persisted_right_get_key(
    _ rightPtr: UnsafeMutableRawPointer?,
    _ outKey: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outKey else {
            throw LABridgeError.invalidArgument("missing output pointer for LAPrivateKey")
        }
        if #available(macOS 13.0, *) {
            let right = try laPersistedRight(rightPtr)
            outKey.pointee = laRetainHandle(LAPrivateKeyHandle(right.key))
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAPersistedRight requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_persisted_right_get_secret")
public func la_persisted_right_get_secret(
    _ rightPtr: UnsafeMutableRawPointer?,
    _ outSecret: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outSecret else {
            throw LABridgeError.invalidArgument("missing output pointer for LASecret")
        }
        if #available(macOS 13.0, *) {
            let right = try laPersistedRight(rightPtr)
            outSecret.pointee = laRetainHandle(LASecretHandle(right.secret))
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAPersistedRight requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_secret_release")
public func la_secret_release(_ ptr: UnsafeMutableRawPointer?) {
    laReleaseHandle(ptr)
}

@_cdecl("la_secret_load_data")
public func la_secret_load_data(
    _ secretPtr: UnsafeMutableRawPointer?,
    _ outBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>?,
    _ outLen: UnsafeMutablePointer<UInt>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        if #available(macOS 13.0, *) {
            let secret = try laSecret(secretPtr)
            let data = try laAwait {
                try await secret.rawData
            }
            return laCopyData(data, outBytes, outLen, errorOut)
        }
        throw LABridgeError.bridgeFailed("LASecret requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_private_key_release")
public func la_private_key_release(_ ptr: UnsafeMutableRawPointer?) {
    laReleaseHandle(ptr)
}

@_cdecl("la_private_key_get_public_key")
public func la_private_key_get_public_key(
    _ keyPtr: UnsafeMutableRawPointer?,
    _ outPublicKey: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outPublicKey else {
            throw LABridgeError.invalidArgument("missing output pointer for LAPublicKey")
        }
        if #available(macOS 13.0, *) {
            let key = try laPrivateKey(keyPtr)
            outPublicKey.pointee = laRetainHandle(LAPublicKeyHandle(key.publicKey))
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAPrivateKey requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_private_key_can_sign_using_algorithm")
public func la_private_key_can_sign_using_algorithm(
    _ keyPtr: UnsafeMutableRawPointer?,
    _ algorithmRaw: UnsafePointer<CChar>?,
    _ outValue: UnsafeMutablePointer<UInt8>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outValue else {
            throw LABridgeError.invalidArgument("missing output pointer for sign capability")
        }
        let algorithm = try laSecKeyAlgorithm(algorithmRaw)
        if #available(macOS 13.0, *) {
            let key = try laPrivateKey(keyPtr)
            outValue.pointee = key.canSign(using: algorithm) ? 1 : 0
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAPrivateKey requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_private_key_sign_data")
public func la_private_key_sign_data(
    _ keyPtr: UnsafeMutableRawPointer?,
    _ data: UnsafePointer<UInt8>?,
    _ dataLen: UInt,
    _ algorithmRaw: UnsafePointer<CChar>?,
    _ outBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>?,
    _ outLen: UnsafeMutablePointer<UInt>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let algorithm = try laSecKeyAlgorithm(algorithmRaw)
        let input = laData(data, len: Int(dataLen))
        if #available(macOS 13.0, *) {
            let key = try laPrivateKey(keyPtr)
            let signature = try laAwait {
                try await key.sign(input, algorithm: algorithm)
            }
            return laCopyData(signature, outBytes, outLen, errorOut)
        }
        throw LABridgeError.bridgeFailed("LAPrivateKey requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_private_key_can_decrypt_using_algorithm")
public func la_private_key_can_decrypt_using_algorithm(
    _ keyPtr: UnsafeMutableRawPointer?,
    _ algorithmRaw: UnsafePointer<CChar>?,
    _ outValue: UnsafeMutablePointer<UInt8>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outValue else {
            throw LABridgeError.invalidArgument("missing output pointer for decrypt capability")
        }
        let algorithm = try laSecKeyAlgorithm(algorithmRaw)
        if #available(macOS 13.0, *) {
            let key = try laPrivateKey(keyPtr)
            outValue.pointee = key.canDecrypt(using: algorithm) ? 1 : 0
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAPrivateKey requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_private_key_decrypt_data")
public func la_private_key_decrypt_data(
    _ keyPtr: UnsafeMutableRawPointer?,
    _ data: UnsafePointer<UInt8>?,
    _ dataLen: UInt,
    _ algorithmRaw: UnsafePointer<CChar>?,
    _ outBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>?,
    _ outLen: UnsafeMutablePointer<UInt>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let algorithm = try laSecKeyAlgorithm(algorithmRaw)
        let input = laData(data, len: Int(dataLen))
        if #available(macOS 13.0, *) {
            let key = try laPrivateKey(keyPtr)
            let plaintext = try laAwait {
                try await key.decrypt(input, algorithm: algorithm)
            }
            return laCopyData(plaintext, outBytes, outLen, errorOut)
        }
        throw LABridgeError.bridgeFailed("LAPrivateKey requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_private_key_can_exchange_keys_using_algorithm")
public func la_private_key_can_exchange_keys_using_algorithm(
    _ keyPtr: UnsafeMutableRawPointer?,
    _ algorithmRaw: UnsafePointer<CChar>?,
    _ outValue: UnsafeMutablePointer<UInt8>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outValue else {
            throw LABridgeError.invalidArgument("missing output pointer for key-exchange capability")
        }
        let algorithm = try laSecKeyAlgorithm(algorithmRaw)
        if #available(macOS 13.0, *) {
            let key = try laPrivateKey(keyPtr)
            outValue.pointee = key.canExchangeKeys(using: algorithm) ? 1 : 0
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAPrivateKey requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_private_key_exchange_keys_with_public_key")
public func la_private_key_exchange_keys_with_public_key(
    _ keyPtr: UnsafeMutableRawPointer?,
    _ publicKey: UnsafePointer<UInt8>?,
    _ publicKeyLen: UInt,
    _ algorithmRaw: UnsafePointer<CChar>?,
    _ requestedSize: Int64,
    _ sharedInfo: UnsafePointer<UInt8>?,
    _ sharedInfoLen: UInt,
    _ hasSharedInfo: UInt8,
    _ outBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>?,
    _ outLen: UnsafeMutablePointer<UInt>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let algorithm = try laSecKeyAlgorithm(algorithmRaw)
        let publicKey = laData(publicKey, len: Int(publicKeyLen))
        if #available(macOS 13.0, *) {
            let key = try laPrivateKey(keyPtr)
            let parameters = try laSecKeyExchangeParameters(
                requestedSize: requestedSize,
                sharedInfo: sharedInfo,
                sharedInfoLen: sharedInfoLen,
                hasSharedInfo: hasSharedInfo
            )
            let sharedSecret = try laAwait {
                try await key.exchangeKeys(
                    publicKey: publicKey,
                    algorithm: algorithm,
                    parameters: parameters
                )
            }
            return laCopyData(sharedSecret, outBytes, outLen, errorOut)
        }
        throw LABridgeError.bridgeFailed("LAPrivateKey requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}
