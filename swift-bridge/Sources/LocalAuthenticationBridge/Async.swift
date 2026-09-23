import Foundation
import LocalAuthentication

// MARK: - Async Policy Evaluation

@_cdecl("la_context_evaluate_policy_async")
public func la_context_evaluate_policy_async(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ policyRaw: Int32,
    _ localizedReason: UnsafePointer<CChar>?,
    _ cb: @convention(c) (UInt8, Int32, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer
) {
    // Resolve all pointer-derived inputs synchronously. The caller (Rust) frees
    // `localizedReason` as soon as this function returns, and the borrowed
    // context handle may be released before the deferred `Task` runs, so they
    // must not be dereferenced inside the asynchronous closure.
    let inputs: Result<(LAContext, LAPolicy, String), Error>
    do {
        inputs = .success((
            try laContext(contextPtr),
            try laPolicy(policyRaw),
            try laRequiredString(localizedReason, name: "localized reason")
        ))
    } catch {
        inputs = .failure(error)
    }

    Task {
        do {
            let (context, policy, reason) = try inputs.get()
            let success = try await context.evaluatePolicy(policy, localizedReason: reason)
            cb(success ? 1 : 0, LA_OK, nil, ctx)
        } catch {
            laDescription(for: error).withCString { errorCString in
                cb(0, laStatus(for: error), errorCString, ctx)
            }
        }
    }
}

// MARK: - Async Access Control Evaluation

@_cdecl("la_context_evaluate_access_control_async")
public func la_context_evaluate_access_control_async(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ accessControlPtr: UnsafeRawPointer?,
    _ operationRaw: Int32,
    _ localizedReason: UnsafePointer<CChar>?,
    _ cb: @convention(c) (UInt8, Int32, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer
) {
    // Resolve all pointer-derived inputs synchronously. The caller (Rust) frees
    // `localizedReason` as soon as this function returns, and the borrowed
    // context handle may be released before the deferred `Task` runs, so they
    // must not be dereferenced inside the asynchronous closure.
    let inputs: Result<(LAContext, SecAccessControl, LAAccessControlOperation, String), Error>
    do {
        let context = try laContext(contextPtr)
        let accessControl = try laAccessControl(accessControlPtr)
        let reason = try laRequiredString(localizedReason, name: "localized reason")
        guard let operation = LAAccessControlOperation(rawValue: Int(operationRaw)) else {
            throw LABridgeError.invalidArgument("unsupported access-control operation: \(operationRaw)")
        }
        inputs = .success((context, accessControl, operation, reason))
    } catch {
        inputs = .failure(error)
    }

    Task {
        do {
            let (context, accessControl, operation, reason) = try inputs.get()
            let success = try await context.evaluateAccessControl(
                accessControl,
                operation: operation,
                localizedReason: reason
            )
            cb(success ? 1 : 0, LA_OK, nil, ctx)
        } catch {
            laDescription(for: error).withCString { errorCString in
                cb(0, laStatus(for: error), errorCString, ctx)
            }
        }
    }
}
