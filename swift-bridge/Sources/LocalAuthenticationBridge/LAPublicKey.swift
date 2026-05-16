import Foundation
import LocalAuthentication
import Security

@available(macOS 13.0, *)
final class LAPublicKeyHandle: LABridgeHandleBase {
    let value: LAPublicKey

    init(_ value: LAPublicKey) {
        self.value = value
    }
}

@available(macOS 13.0, *)
@inline(__always)
func laPublicKey(_ ptr: UnsafeMutableRawPointer?) throws -> LAPublicKey {
    try laBorrowHandle(ptr, as: LAPublicKeyHandle.self, name: "LAPublicKey").value
}

@_cdecl("la_public_key_release")
public func la_public_key_release(_ ptr: UnsafeMutableRawPointer?) {
    laReleaseHandle(ptr)
}

@_cdecl("la_public_key_export_bytes")
public func la_public_key_export_bytes(
    _ keyPtr: UnsafeMutableRawPointer?,
    _ outBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>?,
    _ outLen: UnsafeMutablePointer<UInt>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        if #available(macOS 13.0, *) {
            let key = try laPublicKey(keyPtr)
            let bytes = try laAwait {
                try await key.bytes
            }
            return laCopyData(bytes, outBytes, outLen, errorOut)
        }
        throw LABridgeError.bridgeFailed("LAPublicKey requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_public_key_can_encrypt_using_algorithm")
public func la_public_key_can_encrypt_using_algorithm(
    _ keyPtr: UnsafeMutableRawPointer?,
    _ algorithmRaw: UnsafePointer<CChar>?,
    _ outValue: UnsafeMutablePointer<UInt8>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outValue else {
            throw LABridgeError.invalidArgument("missing output pointer for encryption capability")
        }
        let algorithm = try laSecKeyAlgorithm(algorithmRaw)
        if #available(macOS 13.0, *) {
            let key = try laPublicKey(keyPtr)
            outValue.pointee = key.canEncrypt(using: algorithm) ? 1 : 0
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAPublicKey requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_public_key_encrypt_data")
public func la_public_key_encrypt_data(
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
            let key = try laPublicKey(keyPtr)
            let ciphertext = try laAwait {
                try await key.encrypt(input, algorithm: algorithm)
            }
            return laCopyData(ciphertext, outBytes, outLen, errorOut)
        }
        throw LABridgeError.bridgeFailed("LAPublicKey requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_public_key_can_verify_using_algorithm")
public func la_public_key_can_verify_using_algorithm(
    _ keyPtr: UnsafeMutableRawPointer?,
    _ algorithmRaw: UnsafePointer<CChar>?,
    _ outValue: UnsafeMutablePointer<UInt8>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outValue else {
            throw LABridgeError.invalidArgument("missing output pointer for verification capability")
        }
        let algorithm = try laSecKeyAlgorithm(algorithmRaw)
        if #available(macOS 13.0, *) {
            let key = try laPublicKey(keyPtr)
            outValue.pointee = key.canVerify(using: algorithm) ? 1 : 0
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAPublicKey requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_public_key_verify_data")
public func la_public_key_verify_data(
    _ keyPtr: UnsafeMutableRawPointer?,
    _ signedData: UnsafePointer<UInt8>?,
    _ signedDataLen: UInt,
    _ signature: UnsafePointer<UInt8>?,
    _ signatureLen: UInt,
    _ algorithmRaw: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let algorithm = try laSecKeyAlgorithm(algorithmRaw)
        let signedData = laData(signedData, len: Int(signedDataLen))
        let signature = laData(signature, len: Int(signatureLen))
        if #available(macOS 13.0, *) {
            let key = try laPublicKey(keyPtr)
            try laAwait {
                try await key.verify(signedData, signature: signature, algorithm: algorithm)
            }
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAPublicKey requires macOS 13.0")
    } catch {
        return laFail(error, errorOut)
    }
}
