import Foundation
import LocalAuthentication

// MARK: - Async Policy Evaluation

@_cdecl("la_context_evaluate_policy_async")
public func la_context_evaluate_policy_async(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ policyRaw: Int32,
    _ localizedReason: UnsafePointer<CChar>?,
    _ cb: @convention(c) (UInt8, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer
) {
    Task {
        do {
            let context = try laContext(contextPtr)
            let policy = try laPolicy(policyRaw)
            let reason = try laRequiredString(localizedReason, name: "localized reason")

            let success = try await context.evaluatePolicy(policy, localizedReason: reason)
            cb(success ? 1 : 0, nil, ctx)
        } catch {
            error.localizedDescription.withCString { errorCString in
                cb(0, errorCString, ctx)
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
    _ cb: @convention(c) (UInt8, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer
) {
    Task {
        do {
            let context = try laContext(contextPtr)
            let accessControl = try laAccessControl(accessControlPtr)
            let reason = try laRequiredString(localizedReason, name: "localized reason")
            guard let operation = LAAccessControlOperation(rawValue: Int(operationRaw)) else {
                throw LABridgeError.invalidArgument("unsupported access-control operation: \(operationRaw)")
            }

            let success = try await context.evaluateAccessControl(
                accessControl,
                operation: operation,
                localizedReason: reason
            )
            cb(success ? 1 : 0, nil, ctx)
        } catch {
            error.localizedDescription.withCString { errorCString in
                cb(0, errorCString, ctx)
            }
        }
    }
}
