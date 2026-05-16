# localauthentication-rs coverage audit (vs MacOSX26.2.sdk)

Audit scope: Objective-C symbols in `LocalAuthentication.framework/Headers`, filtered to macOS-available public API. Deprecated macOS symbols are listed as **EXEMPT** and excluded from the coverage denominator per the audit instructions. Crate reachability was checked against public Rust exports under `src/` and the Swift `_cdecl` bridge under `swift-bridge/Sources/LocalAuthenticationBridge/`.

SDK_PUBLIC_SYMBOLS: 139
VERIFIED: 102
GAPS: 28
EXEMPT: 9
COVERAGE_PCT: 78.5%

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `LAContext` | class | `LAContext.h` | `LAContext` |
| `LAContext.biometryType` | property | `LAContext.h` | `LAContext::biometry_type` |
| `LAContext.canEvaluatePolicy:error:` | method | `LAContext.h` | `LAContext::can_evaluate_policy` |
| `LAContext.domainState` | property | `LAContext.h` | `LAContext::domain_state` |
| `LAContext.evaluateAccessControl:operation:localizedReason:reply:` | method | `LAContext.h` | `LAContext::evaluate_access_control_raw (unsafe)` |
| `LAContext.evaluatePolicy:localizedReason:reply:` | method | `LAContext.h` | `LAContext::evaluate_policy` |
| `LAContext.interactionNotAllowed` | property | `LAContext.h` | `LAContext::{interaction_not_allowed, set_interaction_not_allowed}` |
| `LAContext.invalidate` | method | `LAContext.h` | `LAContext::invalidate` |
| `LAContext.isCredentialSet:` | method | `LAContext.h` | `LAContext::is_credential_set` |
| `LAContext.localizedCancelTitle` | property | `LAContext.h` | `LAContext::{localized_cancel_title, set_localized_cancel_title}` |
| `LAContext.localizedFallbackTitle` | property | `LAContext.h` | `LAContext::{localized_fallback_title, set_localized_fallback_title}` |
| `LAContext.localizedReason` | property | `LAContext.h` | `LAContext::{localized_reason, set_localized_reason}` |
| `LAContext.setCredential:type:` | method | `LAContext.h` | `LAContext::{set_credential, clear_credential}` |
| `LAContext.touchIDAuthenticationAllowableReuseDuration` | property | `LAContext.h` | `LAContext::{touch_id_authentication_allowable_reuse_duration, set_touch_id_authentication_allowable_reuse_duration}` |
| `LAPolicy` | enum | `LAContext.h` | `LAPolicy` |
| `LAPolicyDeviceOwnerAuthentication` | enum case | `LAContext.h` | `LAPolicy::DeviceOwnerAuthentication` |
| `LAPolicyDeviceOwnerAuthenticationWithBiometrics` | enum case | `LAContext.h` | `LAPolicy::DeviceOwnerAuthenticationWithBiometrics` |
| `LAPolicyDeviceOwnerAuthenticationWithBiometricsOrCompanion` | enum case | `LAContext.h` | `LAPolicy::DeviceOwnerAuthenticationWithBiometricsOrCompanion` |
| `LAPolicyDeviceOwnerAuthenticationWithCompanion` | enum case | `LAContext.h` | `LAPolicy::DeviceOwnerAuthenticationWithCompanion` |
| `LATouchIDAuthenticationMaximumAllowableReuseDuration` | constant | `LAContext.h` | `LAContext::touch_id_authentication_maximum_allowable_reuse_duration()` |
| `LABiometryType` | enum | `LABiometryType.h` | `BiometryType` |
| `LABiometryTypeFaceID` | enum case | `LABiometryType.h` | `BiometryType::FaceId` |
| `LABiometryTypeNone` | enum case | `LABiometryType.h` | `BiometryType::None` |
| `LABiometryTypeOpticID` | enum case | `LABiometryType.h` | `BiometryType::OpticId` |
| `LABiometryTypeTouchID` | enum case | `LABiometryType.h` | `BiometryType::TouchId` |
| `LACompanionType` | enum | `LACompanionType.h` | `LACompanionType` |
| `LACompanionTypeWatch` | enum case | `LACompanionType.h` | `LACompanionType::Watch` |
| `LAError` | enum | `LAError.h` | `LAError` |
| `LAErrorAppCancel` | enum case | `LAError.h` | `LAError::AppCancel` |
| `LAErrorAuthenticationFailed` | enum case | `LAError.h` | `LAError::AuthenticationFailed` |
| `LAErrorBiometryDisconnected` | enum case | `LAError.h` | `LAError::BiometryDisconnected` |
| `LAErrorBiometryLockout` | enum case | `LAError.h` | `LAError::BiometryLockout` |
| `LAErrorBiometryNotAvailable` | enum case | `LAError.h` | `LAError::BiometryNotAvailable` |
| `LAErrorBiometryNotEnrolled` | enum case | `LAError.h` | `LAError::BiometryNotEnrolled` |
| `LAErrorBiometryNotPaired` | enum case | `LAError.h` | `LAError::BiometryNotPaired` |
| `LAErrorCompanionNotAvailable` | enum case | `LAError.h` | `LAError::CompanionNotAvailable` |
| `LAErrorDomain` | constant | `LAError.h` | `LA_ERROR_DOMAIN / LAError::domain()` |
| `LAErrorInvalidContext` | enum case | `LAError.h` | `LAError::InvalidContext` |
| `LAErrorInvalidDimensions` | enum case | `LAError.h` | `LAError::InvalidDimensions` |
| `LAErrorNotInteractive` | enum case | `LAError.h` | `LAError::NotInteractive` |
| `LAErrorPasscodeNotSet` | enum case | `LAError.h` | `LAError::PasscodeNotSet` |
| `LAErrorSystemCancel` | enum case | `LAError.h` | `LAError::SystemCancel` |
| `LAErrorUserCancel` | enum case | `LAError.h` | `LAError::UserCancel` |
| `LAErrorUserFallback` | enum case | `LAError.h` | `LAError::UserFallback` |
| `LAAuthenticationRequirement` | class | `LARequirement.h` | `LAAuthenticationRequirement` |
| `LAAuthenticationRequirement.biometryCurrentSetRequirement` | property | `LARequirement.h` | `LAAuthenticationRequirement::biometry_current_set_requirement` |
| `LAAuthenticationRequirement.biometryRequirement` | property | `LARequirement.h` | `LAAuthenticationRequirement::biometry_requirement` |
| `LAAuthenticationRequirement.biometryRequirementWithFallback:` | class method | `LARequirement.h` | `LAAuthenticationRequirement::biometry_requirement_with_fallback` |
| `LAAuthenticationRequirement.defaultRequirement` | property | `LARequirement.h` | `LAAuthenticationRequirement::default_requirement` |
| `LABiometryFallbackRequirement` | class | `LARequirement.h` | `LABiometryFallbackRequirement` |
| `LABiometryFallbackRequirement.defaultRequirement` | property | `LARequirement.h` | `LABiometryFallbackRequirement::default_requirement` |
| `LABiometryFallbackRequirement.devicePasscodeRequirement` | property | `LARequirement.h` | `LABiometryFallbackRequirement::device_passcode_requirement` |
| `LARight` | class | `LARight.h` | `LARight` |
| `LARight.authorizeWithLocalizedReason:completion:` | method | `LARight.h` | `LARight::authorize` |
| `LARight.checkCanAuthorizeWithCompletion:` | method | `LARight.h` | `LARight::check_can_authorize` |
| `LARight.deauthorizeWithCompletion:` | method | `LARight.h` | `LARight::deauthorize` |
| `LARight.init` | method | `LARight.h` | `LARight::new` |
| `LARight.initWithRequirement:` | method | `LARight.h` | `LARight::new_with_requirement` |
| `LARight.state` | property | `LARight.h` | `LARight::state` |
| `LARight.tag` | property | `LARight.h` | `LARight::{tag, set_tag}` |
| `LARightState` | enum | `LARight.h` | `LARightState` |
| `LARightStateAuthorized` | enum case | `LARight.h` | `LARightState::Authorized` |
| `LARightStateAuthorizing` | enum case | `LARight.h` | `LARightState::Authorizing` |
| `LARightStateNotAuthorized` | enum case | `LARight.h` | `LARightState::NotAuthorized` |
| `LARightStateUnknown` | enum case | `LARight.h` | `LARightState::Unknown` |
| `LARightStore` | class | `LARightStore.h` | `LARightStore` |
| `LARightStore.removeAllRightsWithCompletion:` | method | `LARightStore.h` | `LARightStore::remove_all_rights` |
| `LARightStore.removeRight:completion:` | method | `LARightStore.h` | `LARightStore::remove_right` |
| `LARightStore.removeRightForIdentifier:completion:` | method | `LARightStore.h` | `LARightStore::remove_right_for_identifier` |
| `LARightStore.rightForIdentifier:completion:` | method | `LARightStore.h` | `LARightStore::right_for_identifier` |
| `LARightStore.saveRight:identifier:completion:` | method | `LARightStore.h` | `LARightStore::save_right` |
| `LARightStore.saveRight:identifier:secret:completion:` | method | `LARightStore.h` | `LARightStore::save_right_with_secret` |
| `LARightStore.sharedStore` | property | `LARightStore.h` | `LARightStore::shared` |
| `LAPersistedRight` | class | `LAPersistedRight.h` | `LAPersistedRight` |
| `LAPersistedRight.key` | property | `LAPersistedRight.h` | `LAPersistedRight::key` |
| `LAPersistedRight.secret` | property | `LAPersistedRight.h` | `LAPersistedRight::secret` |
| `LASecret` | class | `LASecret.h` | `LASecret` |
| `LASecret.loadDataWithCompletion:` | method | `LASecret.h` | `LASecret::load_data` |
| `LAPrivateKey` | class | `LAPrivateKey.h` | `LAPrivateKey` |
| `LAPrivateKey.canDecryptUsingSecKeyAlgorithm:` | method | `LAPrivateKey.h` | `LAPrivateKey::can_decrypt_using` |
| `LAPrivateKey.canExchangeKeysUsingSecKeyAlgorithm:` | method | `LAPrivateKey.h` | `LAPrivateKey::can_exchange_keys_using` |
| `LAPrivateKey.canSignUsingSecKeyAlgorithm:` | method | `LAPrivateKey.h` | `LAPrivateKey::can_sign_using` |
| `LAPrivateKey.decryptData:secKeyAlgorithm:completion:` | method | `LAPrivateKey.h` | `LAPrivateKey::decrypt` |
| `LAPrivateKey.publicKey` | property | `LAPrivateKey.h` | `LAPrivateKey::public_key` |
| `LAPrivateKey.signData:secKeyAlgorithm:completion:` | method | `LAPrivateKey.h` | `LAPrivateKey::sign` |
| `LAPublicKey` | class | `LAPublicKey.h` | `LAPublicKey` |
| `LAPublicKey.canEncryptUsingSecKeyAlgorithm:` | method | `LAPublicKey.h` | `LAPublicKey::can_encrypt_using` |
| `LAPublicKey.canVerifyUsingSecKeyAlgorithm:` | method | `LAPublicKey.h` | `LAPublicKey::can_verify_using` |
| `LAPublicKey.encryptData:secKeyAlgorithm:completion:` | method | `LAPublicKey.h` | `LAPublicKey::encrypt` |
| `LAPublicKey.exportBytesWithCompletion:` | method | `LAPublicKey.h` | `LAPublicKey::export_bytes` |
| `LAPublicKey.verifyData:signature:secKeyAlgorithm:completion:` | method | `LAPublicKey.h` | `LAPublicKey::verify` |
| `LADomainState` | class | `LADomainState.h` | `LADomainState` |
| `LADomainState.biometry` | property | `LADomainState.h` | `LADomainState::biometry` |
| `LADomainState.companion` | property | `LADomainState.h` | `LADomainState::companion` |
| `LADomainState.stateHash` | property | `LADomainState.h` | `LADomainState::state_hash` |
| `LADomainStateBiometry` | class | `LADomainState.h` | `LADomainStateBiometry` |
| `LADomainStateBiometry.biometryType` | property | `LADomainState.h` | `LADomainStateBiometry::biometry_type` |
| `LADomainStateBiometry.stateHash` | property | `LADomainState.h` | `LADomainStateBiometry::state_hash` |
| `LADomainStateCompanion` | class | `LADomainState.h` | `LADomainStateCompanion` |
| `LADomainStateCompanion.availableCompanionTypes` | property | `LADomainState.h` | `LADomainStateCompanion::available_companion_types` |
| `LADomainStateCompanion.stateHash` | property | `LADomainState.h` | `LADomainStateCompanion::state_hash` |
| `LADomainStateCompanion.stateHashForCompanionType:` | method | `LADomainState.h` | `LADomainStateCompanion::state_hash_for` |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| `LAPrivateKey.exchangeKeysWithPublicKey:secKeyAlgorithm:secKeyParameters:completion:` | method | `LAPrivateKey.h` | Missing the actual key-exchange call and a safe SecKey parameter-dictionary wrapper. |
| `LAEnvironment` | class | `LAEnvironment.h` | No Rust/Swift bridge coverage for the macOS 15 environment observer/state surface. |
| `LAEnvironment.addObserver:` | method | `LAEnvironment.h` | No Rust/Swift bridge coverage for the macOS 15 environment observer/state surface. |
| `LAEnvironment.currentUser` | property | `LAEnvironment.h` | No Rust/Swift bridge coverage for the macOS 15 environment observer/state surface. |
| `LAEnvironment.removeObserver:` | method | `LAEnvironment.h` | No Rust/Swift bridge coverage for the macOS 15 environment observer/state surface. |
| `LAEnvironment.state` | property | `LAEnvironment.h` | No Rust/Swift bridge coverage for the macOS 15 environment observer/state surface. |
| `LAEnvironmentObserver` | protocol | `LAEnvironment.h` | No Rust/Swift bridge coverage for the macOS 15 environment observer/state surface. |
| `LAEnvironmentObserver.environment:stateDidChangeFromOldState:` | method | `LAEnvironment.h` | No Rust/Swift bridge coverage for the macOS 15 environment observer/state surface. |
| `LAEnvironmentState` | class | `LAEnvironmentState.h` | No Rust/Swift bridge coverage for the macOS 15 environment observer/state surface. |
| `LAEnvironmentState.allMechanisms` | property | `LAEnvironmentState.h` | No Rust/Swift bridge coverage for the macOS 15 environment observer/state surface. |
| `LAEnvironmentState.biometry` | property | `LAEnvironmentState.h` | No Rust/Swift bridge coverage for the macOS 15 environment observer/state surface. |
| `LAEnvironmentState.companions` | property | `LAEnvironmentState.h` | No Rust/Swift bridge coverage for the macOS 15 environment observer/state surface. |
| `LAEnvironmentState.userPassword` | property | `LAEnvironmentState.h` | No Rust/Swift bridge coverage for the macOS 15 environment observer/state surface. |
| `LAEnvironmentMechanism` | class | `LAEnvironmentMechanism.h` | No Rust/Swift bridge coverage for the macOS 15 environment observer/state surface. |
| `LAEnvironmentMechanism.iconSystemName` | property | `LAEnvironmentMechanism.h` | No Rust/Swift bridge coverage for the macOS 15 environment observer/state surface. |
| `LAEnvironmentMechanism.isUsable` | property | `LAEnvironmentMechanism.h` | No Rust/Swift bridge coverage for the macOS 15 environment observer/state surface. |
| `LAEnvironmentMechanism.localizedName` | property | `LAEnvironmentMechanism.h` | No Rust/Swift bridge coverage for the macOS 15 environment observer/state surface. |
| `LAEnvironmentMechanismBiometry` | class | `LAEnvironmentMechanismBiometry.h` | No Rust/Swift bridge coverage for the macOS 15 environment observer/state surface. |
| `LAEnvironmentMechanismBiometry.biometryType` | property | `LAEnvironmentMechanismBiometry.h` | No Rust/Swift bridge coverage for the macOS 15 environment observer/state surface. |
| `LAEnvironmentMechanismBiometry.builtInSensorInaccessible` | property | `LAEnvironmentMechanismBiometry.h` | No Rust/Swift bridge coverage for the macOS 15 environment observer/state surface. |
| `LAEnvironmentMechanismBiometry.isEnrolled` | property | `LAEnvironmentMechanismBiometry.h` | No Rust/Swift bridge coverage for the macOS 15 environment observer/state surface. |
| `LAEnvironmentMechanismBiometry.isLockedOut` | property | `LAEnvironmentMechanismBiometry.h` | No Rust/Swift bridge coverage for the macOS 15 environment observer/state surface. |
| `LAEnvironmentMechanismBiometry.stateHash` | property | `LAEnvironmentMechanismBiometry.h` | No Rust/Swift bridge coverage for the macOS 15 environment observer/state surface. |
| `LAEnvironmentMechanismCompanion` | class | `LAEnvironmentMechanismCompanion.h` | No Rust/Swift bridge coverage for the macOS 15 environment observer/state surface. |
| `LAEnvironmentMechanismCompanion.stateHash` | property | `LAEnvironmentMechanismCompanion.h` | No Rust/Swift bridge coverage for the macOS 15 environment observer/state surface. |
| `LAEnvironmentMechanismCompanion.type` | property | `LAEnvironmentMechanismCompanion.h` | No Rust/Swift bridge coverage for the macOS 15 environment observer/state surface. |
| `LAEnvironmentMechanismUserPassword` | class | `LAEnvironmentMechanismUserPassword.h` | No Rust/Swift bridge coverage for the macOS 15 environment observer/state surface. |
| `LAEnvironmentMechanismUserPassword.isSet` | property | `LAEnvironmentMechanismUserPassword.h` | No Rust/Swift bridge coverage for the macOS 15 environment observer/state surface. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `LAContext.evaluatedPolicyDomainState` | property | `LAContext.h` | Deprecated in favor of domainState.biometry.stateHash; deliberately excluded from the coverage target. | `API_DEPRECATED_WITH_REPLACEMENT("domainState.biometry.stateHash", macos(10.11, 15.0), ios(9.0, 18.0)) API_UNAVAILABLE(watchos, tvos)` |
| `LAContext.maxBiometryFailures` | property | `LAContext.h` | Deprecated no-op property deliberately excluded from the coverage target. | `API_DEPRECATED("No longer supported", ios(8.3, 9.0), macos(10.10.3, 10.11)) API_UNAVAILABLE(watchos, tvos)` |
| `LAPolicyDeviceOwnerAuthenticationWithBiometricsOrWatch` | enum case | `LAContext.h` | Deprecated watch alias deliberately excluded from the coverage target. | `API_DEPRECATED_WITH_REPLACEMENT("LAPolicyDeviceOwnerAuthenticationWithBiometricsOrCompanion", macos(10.15, 15.0), macCatalyst(13.0, 18.0)) API_UNAVAILABLE(ios, watchos, tvos)` |
| `LAPolicyDeviceOwnerAuthenticationWithWatch` | enum case | `LAContext.h` | Deprecated watch alias deliberately excluded from the coverage target. | `API_DEPRECATED_WITH_REPLACEMENT("LAPolicyDeviceOwnerAuthenticationWithCompanion", macos(10.15, 15.0), macCatalyst(13.0, 18.0)) API_UNAVAILABLE(ios, watchos, tvos)` |
| `LABiometryNone` | enum case | `LABiometryType.h` | Deprecated alias of LABiometryTypeNone deliberately excluded from the coverage target. | `API_DEPRECATED_WITH_REPLACEMENT("LABiometryTypeNone", macos(10.13, 10.13.2), ios(11.0, 11.2))` |
| `LAErrorTouchIDLockout` | enum case | `LAError.h` | Deprecated Touch ID alias deliberately excluded from the coverage target. | `NS_ENUM_DEPRECATED(10_11, 10_13, 9_0, 11_0, "use LAErrorBiometryLockout") __WATCHOS_DEPRECATED(3.0, 4.0, "use LAErrorBiometryLockout") __TVOS_DEPRECATED(10.0, 11.0, "use LAErrorBiometryLockout")` |
| `LAErrorTouchIDNotAvailable` | enum case | `LAError.h` | Deprecated Touch ID alias deliberately excluded from the coverage target. | `NS_ENUM_DEPRECATED(10_10, 10_13, 8_0, 11_0, "use LAErrorBiometryNotAvailable")` |
| `LAErrorTouchIDNotEnrolled` | enum case | `LAError.h` | Deprecated Touch ID alias deliberately excluded from the coverage target. | `NS_ENUM_DEPRECATED(10_10, 10_13, 8_0, 11_0, "use LAErrorBiometryNotEnrolled")` |
| `LAErrorWatchNotAvailable` | enum case | `LAError.h` | Deprecated watch alias deliberately excluded from the coverage target. | `API_DEPRECATED_WITH_REPLACEMENT("LAErrorCompanionNotAvailable", macos(10.15, 15.0)) API_UNAVAILABLE(ios, watchos, tvos)` |
