import Foundation
import LocalAuthentication

@available(macOS 15.0, *)
final class LAEnvironmentHandle: LABridgeHandleBase {
    let value: LAEnvironment

    init(_ value: LAEnvironment) {
        self.value = value
    }
}

@available(macOS 15.0, *)
final class LAEnvironmentStateHandle: LABridgeHandleBase {
    let value: LAEnvironment.State

    init(_ value: LAEnvironment.State) {
        self.value = value
    }
}

@available(macOS 15.0, *)
final class LAEnvironmentMechanismHandle: LABridgeHandleBase {
    let value: LAEnvironment.Mechanism

    init(_ value: LAEnvironment.Mechanism) {
        self.value = value
    }
}

public typealias LARustEnvironmentObserverCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?
) -> Void
public typealias LARustEnvironmentObserverRelease = @convention(c) (UnsafeMutableRawPointer?) -> Void

@available(macOS 15.0, *)
final class LARustEnvironmentObserver: NSObject, LAEnvironment.Observer {
    let callback: LARustEnvironmentObserverCallback
    let releaseContext: LARustEnvironmentObserverRelease
    let context: UnsafeMutableRawPointer?

    init(
        callback: @escaping LARustEnvironmentObserverCallback,
        releaseContext: @escaping LARustEnvironmentObserverRelease,
        context: UnsafeMutableRawPointer?
    ) {
        self.callback = callback
        self.releaseContext = releaseContext
        self.context = context
    }

    deinit {
        releaseContext(context)
    }

    @objc func environment(_ environment: LAEnvironment, stateDidChangeFromOldState oldState: LAEnvironment.State) {
        let environmentHandle = laRetainHandle(LAEnvironmentHandle(environment))
        let oldStateHandle = laRetainHandle(LAEnvironmentStateHandle(oldState))
        callback(context, environmentHandle, oldStateHandle)
    }
}

@available(macOS 15.0, *)
final class LAEnvironmentObserverHandle: LABridgeHandleBase {
    let value: LARustEnvironmentObserver

    init(_ value: LARustEnvironmentObserver) {
        self.value = value
    }
}

@available(macOS 15.0, *)
@inline(__always)
func laEnvironment(_ ptr: UnsafeMutableRawPointer?) throws -> LAEnvironment {
    try laBorrowHandle(ptr, as: LAEnvironmentHandle.self, name: "LAEnvironment").value
}

@available(macOS 15.0, *)
@inline(__always)
func laEnvironmentState(_ ptr: UnsafeMutableRawPointer?) throws -> LAEnvironment.State {
    try laBorrowHandle(ptr, as: LAEnvironmentStateHandle.self, name: "LAEnvironmentState").value
}

@available(macOS 15.0, *)
@inline(__always)
func laEnvironmentMechanism(_ ptr: UnsafeMutableRawPointer?) throws -> LAEnvironment.Mechanism {
    try laBorrowHandle(ptr, as: LAEnvironmentMechanismHandle.self, name: "LAEnvironmentMechanism").value
}

@available(macOS 15.0, *)
@inline(__always)
func laEnvironmentMechanismBiometry(_ ptr: UnsafeMutableRawPointer?) throws -> LAEnvironment.MechanismBiometry {
    let mechanism = try laEnvironmentMechanism(ptr)
    guard let biometry = mechanism as? LAEnvironment.MechanismBiometry else {
        throw LABridgeError.invalidArgument("handle is not LAEnvironmentMechanismBiometry")
    }
    return biometry
}

@available(macOS 15.0, *)
@inline(__always)
func laEnvironmentMechanismCompanion(_ ptr: UnsafeMutableRawPointer?) throws -> LAEnvironment.MechanismCompanion {
    let mechanism = try laEnvironmentMechanism(ptr)
    guard let companion = mechanism as? LAEnvironment.MechanismCompanion else {
        throw LABridgeError.invalidArgument("handle is not LAEnvironmentMechanismCompanion")
    }
    return companion
}

@available(macOS 15.0, *)
@inline(__always)
func laEnvironmentMechanismUserPassword(_ ptr: UnsafeMutableRawPointer?) throws -> LAEnvironment.MechanismUserPassword {
    let mechanism = try laEnvironmentMechanism(ptr)
    guard let userPassword = mechanism as? LAEnvironment.MechanismUserPassword else {
        throw LABridgeError.invalidArgument("handle is not LAEnvironmentMechanismUserPassword")
    }
    return userPassword
}

@available(macOS 15.0, *)
@inline(__always)
func laEnvironmentObserver(_ ptr: UnsafeMutableRawPointer?) throws -> LARustEnvironmentObserver {
    try laBorrowHandle(ptr, as: LAEnvironmentObserverHandle.self, name: "LAEnvironmentObserver").value
}

@available(macOS 15.0, *)
@inline(__always)
func laBiometryTypeValue(_ biometryType: LABiometryType) -> Int32 {
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

@_cdecl("la_environment_current_user")
public func la_environment_current_user(
    _ outEnvironment: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let outEnvironment else {
        return laFail(LABridgeError.invalidArgument("missing output pointer for LAEnvironment"), errorOut)
    }

    if #available(macOS 15.0, *) {
        outEnvironment.pointee = laRetainHandle(LAEnvironmentHandle(.currentUser))
        return LA_OK
    }

    return laFail(LABridgeError.bridgeFailed("LAEnvironment requires macOS 15.0"), errorOut)
}

@_cdecl("la_environment_release")
public func la_environment_release(_ ptr: UnsafeMutableRawPointer?) {
    laReleaseHandle(ptr)
}

@_cdecl("la_environment_get_state")
public func la_environment_get_state(
    _ environmentPtr: UnsafeMutableRawPointer?,
    _ outState: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outState else {
            throw LABridgeError.invalidArgument("missing output pointer for LAEnvironmentState")
        }
        if #available(macOS 15.0, *) {
            let environment = try laEnvironment(environmentPtr)
            outState.pointee = laRetainHandle(LAEnvironmentStateHandle(environment.state))
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAEnvironment requires macOS 15.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_environment_add_observer")
public func la_environment_add_observer(
    _ environmentPtr: UnsafeMutableRawPointer?,
    _ observerPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        if #available(macOS 15.0, *) {
            let environment = try laEnvironment(environmentPtr)
            let observer = try laEnvironmentObserver(observerPtr)
            environment.addObserver(observer)
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAEnvironment requires macOS 15.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_environment_remove_observer")
public func la_environment_remove_observer(
    _ environmentPtr: UnsafeMutableRawPointer?,
    _ observerPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        if #available(macOS 15.0, *) {
            let environment = try laEnvironment(environmentPtr)
            let observer = try laEnvironmentObserver(observerPtr)
            environment.removeObserver(observer)
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAEnvironment requires macOS 15.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_environment_observer_new")
public func la_environment_observer_new(
    _ callback: LARustEnvironmentObserverCallback?,
    _ releaseContext: LARustEnvironmentObserverRelease?,
    _ context: UnsafeMutableRawPointer?,
    _ outObserver: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outObserver else {
            throw LABridgeError.invalidArgument("missing output pointer for LAEnvironmentObserver")
        }
        guard let callback else {
            throw LABridgeError.invalidArgument("missing callback for LAEnvironmentObserver")
        }
        guard let releaseContext else {
            throw LABridgeError.invalidArgument("missing release callback for LAEnvironmentObserver")
        }
        if #available(macOS 15.0, *) {
            let observer = LARustEnvironmentObserver(
                callback: callback,
                releaseContext: releaseContext,
                context: context
            )
            outObserver.pointee = laRetainHandle(LAEnvironmentObserverHandle(observer))
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAEnvironment requires macOS 15.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_environment_observer_release")
public func la_environment_observer_release(_ ptr: UnsafeMutableRawPointer?) {
    laReleaseHandle(ptr)
}

@_cdecl("la_environment_state_release")
public func la_environment_state_release(_ ptr: UnsafeMutableRawPointer?) {
    laReleaseHandle(ptr)
}

@_cdecl("la_environment_state_get_biometry")
public func la_environment_state_get_biometry(
    _ statePtr: UnsafeMutableRawPointer?,
    _ outMechanism: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outMechanism else {
            throw LABridgeError.invalidArgument("missing output pointer for LAEnvironmentMechanismBiometry")
        }
        if #available(macOS 15.0, *) {
            let state = try laEnvironmentState(statePtr)
            outMechanism.pointee = state.biometry.map { laRetainHandle(LAEnvironmentMechanismHandle($0)) }
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAEnvironmentState requires macOS 15.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_environment_state_get_user_password")
public func la_environment_state_get_user_password(
    _ statePtr: UnsafeMutableRawPointer?,
    _ outMechanism: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outMechanism else {
            throw LABridgeError.invalidArgument("missing output pointer for LAEnvironmentMechanismUserPassword")
        }
        if #available(macOS 15.0, *) {
            let state = try laEnvironmentState(statePtr)
            outMechanism.pointee = state.userPassword.map { laRetainHandle(LAEnvironmentMechanismHandle($0)) }
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAEnvironmentState requires macOS 15.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_environment_state_get_companion_count")
public func la_environment_state_get_companion_count(
    _ statePtr: UnsafeMutableRawPointer?,
    _ outCount: UnsafeMutablePointer<Int64>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outCount else {
            throw LABridgeError.invalidArgument("missing output pointer for LAEnvironmentState.companions count")
        }
        if #available(macOS 15.0, *) {
            let state = try laEnvironmentState(statePtr)
            outCount.pointee = Int64(state.companions.count)
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAEnvironmentState requires macOS 15.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_environment_state_get_companion_at")
public func la_environment_state_get_companion_at(
    _ statePtr: UnsafeMutableRawPointer?,
    _ index: Int64,
    _ outMechanism: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outMechanism else {
            throw LABridgeError.invalidArgument("missing output pointer for LAEnvironmentMechanismCompanion")
        }
        if #available(macOS 15.0, *) {
            let state = try laEnvironmentState(statePtr)
            guard index >= 0, index < Int64(state.companions.count) else {
                throw LABridgeError.invalidArgument("companion index out of range")
            }
            outMechanism.pointee = laRetainHandle(LAEnvironmentMechanismHandle(state.companions[Int(index)]))
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAEnvironmentState requires macOS 15.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_environment_state_get_all_mechanism_count")
public func la_environment_state_get_all_mechanism_count(
    _ statePtr: UnsafeMutableRawPointer?,
    _ outCount: UnsafeMutablePointer<Int64>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outCount else {
            throw LABridgeError.invalidArgument("missing output pointer for LAEnvironmentState.allMechanisms count")
        }
        if #available(macOS 15.0, *) {
            let state = try laEnvironmentState(statePtr)
            outCount.pointee = Int64(state.allMechanisms.count)
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAEnvironmentState requires macOS 15.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_environment_state_get_all_mechanism_at")
public func la_environment_state_get_all_mechanism_at(
    _ statePtr: UnsafeMutableRawPointer?,
    _ index: Int64,
    _ outMechanism: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outMechanism else {
            throw LABridgeError.invalidArgument("missing output pointer for LAEnvironmentMechanism")
        }
        if #available(macOS 15.0, *) {
            let state = try laEnvironmentState(statePtr)
            guard index >= 0, index < Int64(state.allMechanisms.count) else {
                throw LABridgeError.invalidArgument("mechanism index out of range")
            }
            outMechanism.pointee = laRetainHandle(LAEnvironmentMechanismHandle(state.allMechanisms[Int(index)]))
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAEnvironmentState requires macOS 15.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_environment_mechanism_release")
public func la_environment_mechanism_release(_ ptr: UnsafeMutableRawPointer?) {
    laReleaseHandle(ptr)
}

@_cdecl("la_environment_mechanism_get_is_usable")
public func la_environment_mechanism_get_is_usable(
    _ mechanismPtr: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<UInt8>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outValue else {
            throw LABridgeError.invalidArgument("missing output pointer for LAEnvironmentMechanism.isUsable")
        }
        if #available(macOS 15.0, *) {
            let mechanism = try laEnvironmentMechanism(mechanismPtr)
            outValue.pointee = mechanism.isUsable ? 1 : 0
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAEnvironmentMechanism requires macOS 15.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_environment_mechanism_get_localized_name")
public func la_environment_mechanism_get_localized_name(
    _ mechanismPtr: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outValue else {
            throw LABridgeError.invalidArgument("missing output pointer for LAEnvironmentMechanism.localizedName")
        }
        if #available(macOS 15.0, *) {
            let mechanism = try laEnvironmentMechanism(mechanismPtr)
            outValue.pointee = laCString(mechanism.localizedName)
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAEnvironmentMechanism requires macOS 15.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_environment_mechanism_get_icon_system_name")
public func la_environment_mechanism_get_icon_system_name(
    _ mechanismPtr: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outValue else {
            throw LABridgeError.invalidArgument("missing output pointer for LAEnvironmentMechanism.iconSystemName")
        }
        if #available(macOS 15.0, *) {
            let mechanism = try laEnvironmentMechanism(mechanismPtr)
            outValue.pointee = laCString(mechanism.iconSystemName)
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAEnvironmentMechanism requires macOS 15.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_environment_mechanism_biometry_get_biometry_type")
public func la_environment_mechanism_biometry_get_biometry_type(
    _ mechanismPtr: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<Int32>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outValue else {
            throw LABridgeError.invalidArgument("missing output pointer for LAEnvironmentMechanismBiometry.biometryType")
        }
        if #available(macOS 15.0, *) {
            let mechanism = try laEnvironmentMechanismBiometry(mechanismPtr)
            outValue.pointee = laBiometryTypeValue(mechanism.biometryType)
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAEnvironmentMechanismBiometry requires macOS 15.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_environment_mechanism_biometry_get_is_enrolled")
public func la_environment_mechanism_biometry_get_is_enrolled(
    _ mechanismPtr: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<UInt8>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outValue else {
            throw LABridgeError.invalidArgument("missing output pointer for LAEnvironmentMechanismBiometry.isEnrolled")
        }
        if #available(macOS 15.0, *) {
            let mechanism = try laEnvironmentMechanismBiometry(mechanismPtr)
            outValue.pointee = mechanism.isEnrolled ? 1 : 0
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAEnvironmentMechanismBiometry requires macOS 15.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_environment_mechanism_biometry_get_is_locked_out")
public func la_environment_mechanism_biometry_get_is_locked_out(
    _ mechanismPtr: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<UInt8>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outValue else {
            throw LABridgeError.invalidArgument("missing output pointer for LAEnvironmentMechanismBiometry.isLockedOut")
        }
        if #available(macOS 15.0, *) {
            let mechanism = try laEnvironmentMechanismBiometry(mechanismPtr)
            outValue.pointee = mechanism.isLockedOut ? 1 : 0
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAEnvironmentMechanismBiometry requires macOS 15.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_environment_mechanism_biometry_get_state_hash")
public func la_environment_mechanism_biometry_get_state_hash(
    _ mechanismPtr: UnsafeMutableRawPointer?,
    _ outBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>?,
    _ outLen: UnsafeMutablePointer<UInt>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        if #available(macOS 15.0, *) {
            let mechanism = try laEnvironmentMechanismBiometry(mechanismPtr)
            return laCopyData(mechanism.stateHash, outBytes, outLen, errorOut)
        }
        throw LABridgeError.bridgeFailed("LAEnvironmentMechanismBiometry requires macOS 15.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_environment_mechanism_biometry_get_built_in_sensor_inaccessible")
public func la_environment_mechanism_biometry_get_built_in_sensor_inaccessible(
    _ mechanismPtr: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<UInt8>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outValue else {
            throw LABridgeError.invalidArgument("missing output pointer for LAEnvironmentMechanismBiometry.builtInSensorInaccessible")
        }
        if #available(macOS 15.0, *) {
            let mechanism = try laEnvironmentMechanismBiometry(mechanismPtr)
            outValue.pointee = mechanism.builtInSensorInaccessible ? 1 : 0
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAEnvironmentMechanismBiometry requires macOS 15.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_environment_mechanism_companion_get_type")
public func la_environment_mechanism_companion_get_type(
    _ mechanismPtr: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<Int32>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outValue else {
            throw LABridgeError.invalidArgument("missing output pointer for LAEnvironmentMechanismCompanion.type")
        }
        if #available(macOS 15.0, *) {
            let mechanism = try laEnvironmentMechanismCompanion(mechanismPtr)
            outValue.pointee = laCompanionTypeValue(mechanism.type)
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAEnvironmentMechanismCompanion requires macOS 15.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_environment_mechanism_companion_get_state_hash")
public func la_environment_mechanism_companion_get_state_hash(
    _ mechanismPtr: UnsafeMutableRawPointer?,
    _ outBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>?,
    _ outLen: UnsafeMutablePointer<UInt>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        if #available(macOS 15.0, *) {
            let mechanism = try laEnvironmentMechanismCompanion(mechanismPtr)
            guard let stateHash = mechanism.stateHash else {
                outBytes?.pointee = nil
                outLen?.pointee = 0
                return LA_OK
            }
            return laCopyData(stateHash, outBytes, outLen, errorOut)
        }
        throw LABridgeError.bridgeFailed("LAEnvironmentMechanismCompanion requires macOS 15.0")
    } catch {
        return laFail(error, errorOut)
    }
}

@_cdecl("la_environment_mechanism_user_password_get_is_set")
public func la_environment_mechanism_user_password_get_is_set(
    _ mechanismPtr: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<UInt8>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard let outValue else {
            throw LABridgeError.invalidArgument("missing output pointer for LAEnvironmentMechanismUserPassword.isSet")
        }
        if #available(macOS 15.0, *) {
            let mechanism = try laEnvironmentMechanismUserPassword(mechanismPtr)
            outValue.pointee = mechanism.isSet ? 1 : 0
            return LA_OK
        }
        throw LABridgeError.bridgeFailed("LAEnvironmentMechanismUserPassword requires macOS 15.0")
    } catch {
        return laFail(error, errorOut)
    }
}
