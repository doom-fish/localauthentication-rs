import Foundation
import Security

let LA_OK: Int32 = 0
let LA_INVALID_ARGUMENT: Int32 = -10_000
let LA_TIMED_OUT: Int32 = -10_001
let LA_BRIDGE_FAILED: Int32 = -10_002
let LA_UNKNOWN: Int32 = -10_099

@available(macOS 10.13.2, *)
let LA_BIOMETRY_NONE: Int32 = 0
@available(macOS 10.13.2, *)
let LA_BIOMETRY_TOUCH_ID: Int32 = 1
@available(macOS 10.13.2, *)
let LA_BIOMETRY_FACE_ID: Int32 = 2
@available(macOS 14.0, *)
let LA_BIOMETRY_OPTIC_ID: Int32 = 4

let LA_COMPANION_WATCH: Int32 = 1
let LA_COMPANION_MAC: Int32 = 2
let LA_COMPANION_VISION: Int32 = 4

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

final class ResultHolder<Value> {
    private let lock = NSLock()
    private var _result: Result<Value, Error>?

    var result: Result<Value, Error>? {
        get {
            lock.lock()
            defer { lock.unlock() }
            return _result
        }
        set {
            lock.lock()
            defer { lock.unlock() }
            _result = newValue
        }
    }
}

class LABridgeHandleBase: NSObject {}

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
    _ error: Error,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let status: Int32
    if let bridgeError = error as? LABridgeError {
        status = bridgeError.statusCode
        laWriteError(errorOut, bridgeError.localizedDescription)
    } else {
        let nsError = error as NSError
        status = Int32(nsError.code)
        laWriteError(errorOut, nsError.localizedDescription)
    }
    return status
}

@inline(__always)
func laRetainHandle<T: LABridgeHandleBase>(_ object: T) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
func laBorrowHandle<T: LABridgeHandleBase>(
    _ ptr: UnsafeMutableRawPointer?,
    as type: T.Type,
    name: String
) throws -> T {
    guard let ptr else {
        throw LABridgeError.invalidArgument("missing `\(name)` handle")
    }
    let typed = ptr.assumingMemoryBound(to: T.self)
    return Unmanaged<T>.fromOpaque(UnsafeRawPointer(typed)).takeUnretainedValue()
}

@inline(__always)
func laReleaseHandle(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    let typed = ptr.assumingMemoryBound(to: LABridgeHandleBase.self)
    Unmanaged<LABridgeHandleBase>.fromOpaque(UnsafeRawPointer(typed)).release()
}

@inline(__always)
func laOptionalString(_ value: UnsafePointer<CChar>?) -> String? {
    guard let value else { return nil }
    return String(cString: value)
}

@inline(__always)
func laRequiredString(_ value: UnsafePointer<CChar>?, name: String) throws -> String {
    guard let value else {
        throw LABridgeError.invalidArgument("\(name) must not be null")
    }
    let string = String(cString: value)
    guard !string.isEmpty else {
        throw LABridgeError.invalidArgument("\(name) must not be empty")
    }
    return string
}

@inline(__always)
func laData(_ bytes: UnsafePointer<UInt8>?, len: Int) -> Data {
    guard len > 0 else {
        return Data()
    }
    guard let bytes else {
        return Data()
    }
    return Data(bytes: bytes, count: len)
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
            LABridgeError.invalidArgument("missing output pointers for byte buffer"),
            errorOut
        )
    }

    outLen.pointee = UInt(data.count)
    if data.isEmpty {
        outBytes.pointee = nil
        return LA_OK
    }

    guard let raw = malloc(data.count) else {
        return laFail(LABridgeError.bridgeFailed("malloc failed"), errorOut)
    }
    let buffer = raw.assumingMemoryBound(to: UInt8.self)
    data.copyBytes(to: buffer, count: data.count)
    outBytes.pointee = buffer
    return LA_OK
}

@inline(__always)
func laCopyInt32Array(
    _ values: [Int32],
    _ outValues: UnsafeMutablePointer<UnsafeMutablePointer<Int32>?>?,
    _ outLen: UnsafeMutablePointer<UInt>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let outValues, let outLen else {
        return laFail(
            LABridgeError.invalidArgument("missing output pointers for int32 array"),
            errorOut
        )
    }

    outLen.pointee = UInt(values.count)
    if values.isEmpty {
        outValues.pointee = nil
        return LA_OK
    }

    let count = values.count
    guard let raw = malloc(count * MemoryLayout<Int32>.stride) else {
        return laFail(LABridgeError.bridgeFailed("malloc failed"), errorOut)
    }
    let buffer = raw.assumingMemoryBound(to: Int32.self)
    for (index, value) in values.enumerated() {
        buffer.advanced(by: index).pointee = value
    }
    outValues.pointee = buffer
    return LA_OK
}

func laAwait<Value>(
    timeout: TimeInterval = 30,
    _ operation: @escaping () async throws -> Value
) throws -> Value {
    let semaphore = DispatchSemaphore(value: 0)
    let holder = ResultHolder<Value>()

    Task {
        do {
            holder.result = .success(try await operation())
        } catch {
            holder.result = .failure(error)
        }
        semaphore.signal()
    }

    if semaphore.wait(timeout: .now() + timeout) == .timedOut {
        throw LABridgeError.timedOut("LocalAuthentication operation timed out")
    }

    guard let result = holder.result else {
        throw LABridgeError.unknown("LocalAuthentication operation completed without a result")
    }

    return try result.get()
}

@inline(__always)
func laSecKeyAlgorithm(_ rawValue: UnsafePointer<CChar>?) throws -> SecKeyAlgorithm {
    let rawName = try laRequiredString(rawValue, name: "algorithm")
    return SecKeyAlgorithm(rawValue: rawName as CFString)
}
