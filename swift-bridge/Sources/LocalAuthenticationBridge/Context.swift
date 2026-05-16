import Foundation
import LocalAuthentication

@_cdecl("la_context_new")
public func la_context_new(
    _ outContext: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let outContext else {
        return laFail(.invalidArgument("missing output pointer for `LAContext`"), errorOut)
    }

    outContext.pointee = laRetain(LAContext())
    return LA_OK
}

@_cdecl("la_context_invalidate")
public func la_context_invalidate(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let context = try laContext(contextPtr)
        context.invalidate()
        return LA_OK
    } catch let error as LABridgeError {
        return laFail(error, errorOut)
    } catch {
        return laFail(.unknown(error.localizedDescription), errorOut)
    }
}

@_cdecl("la_context_can_evaluate_policy")
public func la_context_can_evaluate_policy(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ policyRaw: Int32,
    _ outCanEvaluate: UnsafeMutablePointer<UInt8>?,
    _ outErrorCode: UnsafeMutablePointer<Int32>?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outCanEvaluate, let outErrorCode else {
            throw LABridgeError.invalidArgument("missing output pointers for policy preflight")
        }

        let context = try laContext(contextPtr)
        let policy = try laPolicy(policyRaw)
        var frameworkError: NSError?
        let canEvaluate = context.canEvaluatePolicy(policy, error: &frameworkError)
        outCanEvaluate.pointee = canEvaluate ? 1 : 0
        laWriteFrameworkError(frameworkError, outErrorCode, outErrorMessage)
        return LA_OK
    } catch let error as LABridgeError {
        return laFail(error, errorOut)
    } catch {
        return laFail(.unknown(error.localizedDescription), errorOut)
    }
}

@_cdecl("la_context_evaluate_policy")
public func la_context_evaluate_policy(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ policyRaw: Int32,
    _ localizedReason: UnsafePointer<CChar>?,
    _ outSuccess: UnsafeMutablePointer<UInt8>?,
    _ outErrorCode: UnsafeMutablePointer<Int32>?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outSuccess, let outErrorCode else {
            throw LABridgeError.invalidArgument("missing output pointers for policy evaluation")
        }

        let context = try laContext(contextPtr)
        let policy = try laPolicy(policyRaw)
        guard let localizedReason else {
            throw LABridgeError.invalidArgument("localized reason must not be null")
        }

        let reason = String(cString: localizedReason)
        guard !reason.isEmpty else {
            throw LABridgeError.invalidArgument("localized reason must not be empty")
        }

        let semaphore = DispatchSemaphore(value: 0)
        var evaluationSucceeded = false
        var frameworkError: NSError?

        context.evaluatePolicy(policy, localizedReason: reason) { success, error in
            evaluationSucceeded = success
            frameworkError = error as NSError?
            semaphore.signal()
        }

        if semaphore.wait(timeout: .now() + .seconds(300)) == .timedOut {
            throw LABridgeError.timedOut("`LocalAuthentication` evaluation timed out")
        }

        outSuccess.pointee = evaluationSucceeded ? 1 : 0
        laWriteFrameworkError(frameworkError, outErrorCode, outErrorMessage)
        return LA_OK
    } catch let error as LABridgeError {
        return laFail(error, errorOut)
    } catch {
        return laFail(.unknown(error.localizedDescription), errorOut)
    }
}

@_cdecl("la_context_get_localized_fallback_title")
public func la_context_get_localized_fallback_title(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outTitle: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outTitle else {
            throw LABridgeError.invalidArgument("missing output pointer for fallback title")
        }

        let context = try laContext(contextPtr)
        outTitle.pointee = context.localizedFallbackTitle.flatMap(laCString)
        return LA_OK
    } catch let error as LABridgeError {
        return laFail(error, errorOut)
    } catch {
        return laFail(.unknown(error.localizedDescription), errorOut)
    }
}

@_cdecl("la_context_set_localized_fallback_title")
public func la_context_set_localized_fallback_title(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ title: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let context = try laContext(contextPtr)
        context.localizedFallbackTitle = laOptionalString(title)
        return LA_OK
    } catch let error as LABridgeError {
        return laFail(error, errorOut)
    } catch {
        return laFail(.unknown(error.localizedDescription), errorOut)
    }
}

@_cdecl("la_context_get_localized_cancel_title")
public func la_context_get_localized_cancel_title(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outTitle: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outTitle else {
            throw LABridgeError.invalidArgument("missing output pointer for cancel title")
        }

        let context = try laContext(contextPtr)
        guard #available(macOS 10.12, *) else {
            throw LABridgeError.bridgeFailed("localizedCancelTitle requires macOS 10.12")
        }

        outTitle.pointee = context.localizedCancelTitle.flatMap(laCString)
        return LA_OK
    } catch let error as LABridgeError {
        return laFail(error, errorOut)
    } catch {
        return laFail(.unknown(error.localizedDescription), errorOut)
    }
}

@_cdecl("la_context_set_localized_cancel_title")
public func la_context_set_localized_cancel_title(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ title: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let context = try laContext(contextPtr)
        guard #available(macOS 10.12, *) else {
            throw LABridgeError.bridgeFailed("localizedCancelTitle requires macOS 10.12")
        }

        context.localizedCancelTitle = laOptionalString(title)
        return LA_OK
    } catch let error as LABridgeError {
        return laFail(error, errorOut)
    } catch {
        return laFail(.unknown(error.localizedDescription), errorOut)
    }
}

@_cdecl("la_context_get_allowable_reuse_duration")
public func la_context_get_allowable_reuse_duration(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outDuration: UnsafeMutablePointer<Double>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outDuration else {
            throw LABridgeError.invalidArgument("missing output pointer for reuse duration")
        }

        let context = try laContext(contextPtr)
        guard #available(macOS 10.12, *) else {
            throw LABridgeError.bridgeFailed(
                "touchIDAuthenticationAllowableReuseDuration requires macOS 10.12"
            )
        }

        outDuration.pointee = context.touchIDAuthenticationAllowableReuseDuration
        return LA_OK
    } catch let error as LABridgeError {
        return laFail(error, errorOut)
    } catch {
        return laFail(.unknown(error.localizedDescription), errorOut)
    }
}

@_cdecl("la_context_set_allowable_reuse_duration")
public func la_context_set_allowable_reuse_duration(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ duration: Double,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard duration.isFinite, duration >= 0 else {
            throw LABridgeError.invalidArgument(
                "allowable reuse duration must be a finite, non-negative number"
            )
        }

        let context = try laContext(contextPtr)
        guard #available(macOS 10.12, *) else {
            throw LABridgeError.bridgeFailed(
                "touchIDAuthenticationAllowableReuseDuration requires macOS 10.12"
            )
        }

        context.touchIDAuthenticationAllowableReuseDuration = duration
        return LA_OK
    } catch let error as LABridgeError {
        return laFail(error, errorOut)
    } catch {
        return laFail(.unknown(error.localizedDescription), errorOut)
    }
}

@_cdecl("la_context_get_interaction_not_allowed")
public func la_context_get_interaction_not_allowed(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<UInt8>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outValue else {
            throw LABridgeError.invalidArgument(
                "missing output pointer for interactionNotAllowed"
            )
        }

        let context = try laContext(contextPtr)
        guard #available(macOS 10.13, *) else {
            throw LABridgeError.bridgeFailed("interactionNotAllowed requires macOS 10.13")
        }

        outValue.pointee = context.interactionNotAllowed ? 1 : 0
        return LA_OK
    } catch let error as LABridgeError {
        return laFail(error, errorOut)
    } catch {
        return laFail(.unknown(error.localizedDescription), errorOut)
    }
}

@_cdecl("la_context_set_interaction_not_allowed")
public func la_context_set_interaction_not_allowed(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ value: UInt8,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let context = try laContext(contextPtr)
        guard #available(macOS 10.13, *) else {
            throw LABridgeError.bridgeFailed("interactionNotAllowed requires macOS 10.13")
        }

        context.interactionNotAllowed = value != 0
        return LA_OK
    } catch let error as LABridgeError {
        return laFail(error, errorOut)
    } catch {
        return laFail(.unknown(error.localizedDescription), errorOut)
    }
}

@_cdecl("la_context_get_biometry_type")
public func la_context_get_biometry_type(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outBiometryType: UnsafeMutablePointer<Int32>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outBiometryType else {
            throw LABridgeError.invalidArgument("missing output pointer for biometry type")
        }

        let context = try laContext(contextPtr)
        outBiometryType.pointee = laBiometryTypeValue(for: context)
        return LA_OK
    } catch let error as LABridgeError {
        return laFail(error, errorOut)
    } catch {
        return laFail(.unknown(error.localizedDescription), errorOut)
    }
}

@_cdecl("la_context_get_evaluated_policy_domain_state")
public func la_context_get_evaluated_policy_domain_state(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>?,
    _ outLen: UnsafeMutablePointer<UInt>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outBytes, let outLen else {
            throw LABridgeError.invalidArgument(
                "missing output pointers for evaluated policy domain state"
            )
        }

        let context = try laContext(contextPtr)
        guard let state = context.evaluatedPolicyDomainState else {
            outBytes.pointee = nil
            outLen.pointee = 0
            return LA_OK
        }

        return laCopyData(state, outBytes, outLen, errorOut)
    } catch let error as LABridgeError {
        return laFail(error, errorOut)
    } catch {
        return laFail(.unknown(error.localizedDescription), errorOut)
    }
}
