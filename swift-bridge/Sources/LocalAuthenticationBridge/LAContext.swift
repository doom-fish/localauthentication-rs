import Foundation
import LocalAuthentication
import Security

final class LAContextHandle: LABridgeHandleBase {
    let value: LAContext

    init(_ value: LAContext) {
        self.value = value
    }
}

@inline(__always)
func laContext(_ ptr: UnsafeMutableRawPointer?) throws -> LAContext {
    try laBorrowHandle(ptr, as: LAContextHandle.self, name: "LAContext").value
}

@inline(__always)
func laAccessControl(_ ptr: UnsafeRawPointer?) throws -> SecAccessControl {
    guard let ptr else {
        throw LABridgeError.invalidArgument("access control pointer must not be null")
    }
    return unsafeBitCast(ptr, to: SecAccessControl.self)
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

@available(macOS 15.0, *)
@inline(__always)
func laCompanionType(_ raw: Int32) throws -> LACompanionType {
    switch raw {
    case LA_COMPANION_WATCH:
        return .watch
    default:
        throw LABridgeError.invalidArgument("unsupported companion type on macOS: \(raw)")
    }
}

@available(macOS 15.0, *)
@inline(__always)
func laCompanionTypeValue(_ companionType: LACompanionType) -> Int32 {
    switch companionType {
    case .watch:
        return LA_COMPANION_WATCH
    @unknown default:
        return 0
    }
}

@_cdecl("la_context_new")
public func la_context_new(
    _ outContext: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let outContext else {
        return laFail(LABridgeError.invalidArgument("missing output pointer for LAContext"), errorOut)
    }

    outContext.pointee = laRetainHandle(LAContextHandle(LAContext()))
    return LA_OK
}

@_cdecl("la_context_release")
public func la_context_release(_ ptr: UnsafeMutableRawPointer?) {
    laReleaseHandle(ptr)
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
    } catch {
        return laFail(error, errorOut)
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
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_context_evaluate_policy")
public func la_context_evaluate_policy(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ policyRaw: Int32,
    _ localizedReason: UnsafePointer<CChar>?,
    _ outSuccess: UnsafeMutablePointer<UInt8>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outSuccess else {
            throw LABridgeError.invalidArgument("missing output pointer for policy evaluation")
        }

        let context = try laContext(contextPtr)
        let policy = try laPolicy(policyRaw)
        let reason = try laRequiredString(localizedReason, name: "localized reason")
        let success = try laAwait {
            try await context.evaluatePolicy(policy, localizedReason: reason)
        }
        outSuccess.pointee = success ? 1 : 0
        return LA_OK
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_context_evaluate_access_control")
public func la_context_evaluate_access_control(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ accessControlPtr: UnsafeRawPointer?,
    _ operationRaw: Int32,
    _ localizedReason: UnsafePointer<CChar>?,
    _ outSuccess: UnsafeMutablePointer<UInt8>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outSuccess else {
            throw LABridgeError.invalidArgument("missing output pointer for access-control evaluation")
        }

        let context = try laContext(contextPtr)
        let accessControl = try laAccessControl(accessControlPtr)
        let reason = try laRequiredString(localizedReason, name: "localized reason")
        guard let operation = LAAccessControlOperation(rawValue: Int(operationRaw)) else {
            throw LABridgeError.invalidArgument("unsupported access-control operation: \(operationRaw)")
        }

        let success = try laAwait {
            try await context.evaluateAccessControl(
                accessControl,
                operation: operation,
                localizedReason: reason
            )
        }
        outSuccess.pointee = success ? 1 : 0
        return LA_OK
    } catch {
        return laFail(error, errorOut)
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
    } catch {
        return laFail(error, errorOut)
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
    } catch {
        return laFail(error, errorOut)
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
        if #available(macOS 10.12, *) {
            outTitle.pointee = context.localizedCancelTitle.flatMap(laCString)
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("localizedCancelTitle requires macOS 10.12")
    } catch {
        return laFail(error, errorOut)
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
        if #available(macOS 10.12, *) {
            context.localizedCancelTitle = laOptionalString(title)
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("localizedCancelTitle requires macOS 10.12")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_context_get_localized_reason")
public func la_context_get_localized_reason(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outReason: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outReason else {
            throw LABridgeError.invalidArgument("missing output pointer for localized reason")
        }

        let context = try laContext(contextPtr)
        if #available(macOS 10.13, *) {
            outReason.pointee = laCString(context.localizedReason)
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("localizedReason requires macOS 10.13")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_context_set_localized_reason")
public func la_context_set_localized_reason(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ localizedReason: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let context = try laContext(contextPtr)
        let reason = try laRequiredString(localizedReason, name: "localized reason")
        if #available(macOS 10.13, *) {
            context.localizedReason = reason
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("localizedReason requires macOS 10.13")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_context_get_touch_id_authentication_allowable_reuse_duration")
public func la_context_get_touch_id_authentication_allowable_reuse_duration(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outDuration: UnsafeMutablePointer<Double>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outDuration else {
            throw LABridgeError.invalidArgument("missing output pointer for reuse duration")
        }

        let context = try laContext(contextPtr)
        if #available(macOS 10.12, *) {
            outDuration.pointee = context.touchIDAuthenticationAllowableReuseDuration
            return LA_OK
        }
        throw LABridgeError.bridgeFailed(
            "touchIDAuthenticationAllowableReuseDuration requires macOS 10.12"
        )
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_context_set_touch_id_authentication_allowable_reuse_duration")
public func la_context_set_touch_id_authentication_allowable_reuse_duration(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ duration: Double,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        if !duration.isFinite || duration < 0 {
            throw LABridgeError.invalidArgument(
                "allowable reuse duration must be a finite, non-negative number"
            )
        }

        let context = try laContext(contextPtr)
        if #available(macOS 10.12, *) {
            context.touchIDAuthenticationAllowableReuseDuration = duration
            return LA_OK
        }
        throw LABridgeError.bridgeFailed(
            "touchIDAuthenticationAllowableReuseDuration requires macOS 10.12"
        )
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_context_get_touch_id_authentication_maximum_allowable_reuse_duration")
public func la_context_get_touch_id_authentication_maximum_allowable_reuse_duration() -> Double {
    if #available(macOS 10.12, *) {
        return LATouchIDAuthenticationMaximumAllowableReuseDuration
    }
    return 0
}

@_cdecl("la_context_get_interaction_not_allowed")
public func la_context_get_interaction_not_allowed(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<UInt8>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outValue else {
            throw LABridgeError.invalidArgument("missing output pointer for interactionNotAllowed")
        }

        let context = try laContext(contextPtr)
        if #available(macOS 10.13, *) {
            outValue.pointee = context.interactionNotAllowed ? 1 : 0
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("interactionNotAllowed requires macOS 10.13")
    } catch {
        return laFail(error, errorOut)
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
        if #available(macOS 10.13, *) {
            context.interactionNotAllowed = value != 0
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("interactionNotAllowed requires macOS 10.13")
    } catch {
        return laFail(error, errorOut)
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
    } catch {
        return laFail(error, errorOut)
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
        let context = try laContext(contextPtr)
        guard let state = context.evaluatedPolicyDomainState else {
            outBytes?.pointee = nil
            outLen?.pointee = 0
            return LA_OK
        }
        return laCopyData(state, outBytes, outLen, errorOut)
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_context_set_credential")
public func la_context_set_credential(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ credentialBytes: UnsafePointer<UInt8>?,
    _ credentialLen: UInt,
    _ credentialTypeRaw: Int32,
    _ hasCredential: UInt8,
    _ outWasSet: UnsafeMutablePointer<UInt8>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outWasSet else {
            throw LABridgeError.invalidArgument("missing output pointer for credential result")
        }

        let context = try laContext(contextPtr)
        let credentialType = try laCredentialType(credentialTypeRaw)
        let credential = laCredentialData(
            credentialBytes,
            len: Int(credentialLen),
            hasCredential: hasCredential != 0
        )
        let success = context.setCredential(credential, type: credentialType)
        outWasSet.pointee = success ? 1 : 0
        return LA_OK
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_context_is_credential_set")
public func la_context_is_credential_set(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ credentialTypeRaw: Int32,
    _ outIsSet: UnsafeMutablePointer<UInt8>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outIsSet else {
            throw LABridgeError.invalidArgument("missing output pointer for credential query")
        }

        let context = try laContext(contextPtr)
        let credentialType = try laCredentialType(credentialTypeRaw)
        outIsSet.pointee = context.isCredentialSet(credentialType) ? 1 : 0
        return LA_OK
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_context_get_domain_state_hash")
public func la_context_get_domain_state_hash(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>?,
    _ outLen: UnsafeMutablePointer<UInt>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let context = try laContext(contextPtr)
        if #available(macOS 15.0, *) {
            guard let state = context.domainState.stateHash else {
                outBytes?.pointee = nil
                outLen?.pointee = 0
                return LA_OK
            }
            return laCopyData(state, outBytes, outLen, errorOut)
        }
        throw LABridgeError.bridgeFailed("domainState requires macOS 15.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_context_get_domain_state_biometry_type")
public func la_context_get_domain_state_biometry_type(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outBiometryType: UnsafeMutablePointer<Int32>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outBiometryType else {
            throw LABridgeError.invalidArgument("missing output pointer for domain-state biometry type")
        }

        let context = try laContext(contextPtr)
        if #available(macOS 15.0, *) {
            let biometry = context.domainState.biometry
            switch biometry.biometryType {
            case .none:
                outBiometryType.pointee = LA_BIOMETRY_NONE
            case .touchID:
                outBiometryType.pointee = LA_BIOMETRY_TOUCH_ID
            case .faceID:
                outBiometryType.pointee = LA_BIOMETRY_FACE_ID
            case .opticID:
                outBiometryType.pointee = LA_BIOMETRY_OPTIC_ID
            @unknown default:
                outBiometryType.pointee = LA_BIOMETRY_NONE
            }
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("domainState requires macOS 15.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_context_get_domain_state_biometry_hash")
public func la_context_get_domain_state_biometry_hash(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>?,
    _ outLen: UnsafeMutablePointer<UInt>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let context = try laContext(contextPtr)
        if #available(macOS 15.0, *) {
            guard let state = context.domainState.biometry.stateHash else {
                outBytes?.pointee = nil
                outLen?.pointee = 0
                return LA_OK
            }
            return laCopyData(state, outBytes, outLen, errorOut)
        }
        throw LABridgeError.bridgeFailed("domainState requires macOS 15.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_context_get_domain_state_companion_types")
public func la_context_get_domain_state_companion_types(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outValues: UnsafeMutablePointer<UnsafeMutablePointer<Int32>?>?,
    _ outLen: UnsafeMutablePointer<UInt>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let context = try laContext(contextPtr)
        if #available(macOS 15.0, *) {
            let values = context.domainState.companion.availableCompanionTypes.map(laCompanionTypeValue)
            return laCopyInt32Array(values.sorted(), outValues, outLen, errorOut)
        }
        throw LABridgeError.bridgeFailed("domainState.companion requires macOS 15.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_context_get_domain_state_companion_hash")
public func la_context_get_domain_state_companion_hash(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>?,
    _ outLen: UnsafeMutablePointer<UInt>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let context = try laContext(contextPtr)
        if #available(macOS 15.0, *) {
            guard let state = context.domainState.companion.stateHash else {
                outBytes?.pointee = nil
                outLen?.pointee = 0
                return LA_OK
            }
            return laCopyData(state, outBytes, outLen, errorOut)
        }
        throw LABridgeError.bridgeFailed("domainState.companion requires macOS 15.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_context_get_domain_state_companion_hash_for_type")
public func la_context_get_domain_state_companion_hash_for_type(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ companionTypeRaw: Int32,
    _ outBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>?,
    _ outLen: UnsafeMutablePointer<UInt>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let context = try laContext(contextPtr)
        if #available(macOS 15.0, *) {
            let companionType = try laCompanionType(companionTypeRaw)
            guard let state = context.domainState.companion.stateHash(for: companionType) else {
                outBytes?.pointee = nil
                outLen?.pointee = 0
                return LA_OK
            }
            return laCopyData(state, outBytes, outLen, errorOut)
        }
        throw LABridgeError.bridgeFailed("domainState.companion requires macOS 15.0")
    } catch {
        return laFail(error, errorOut)
    }
}
