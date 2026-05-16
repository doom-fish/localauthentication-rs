# LocalAuthentication.framework coverage audit

Audited against `MacOSX26.2.sdk/System/Library/Frameworks/LocalAuthentication.framework/Headers`.

Legend:

- ✅ implemented
- 🟡 partial
- ⏭️ skipped / deferred

> Note: Apple does not ship a separate `LACredential.h`; the credential surface lives in `LAContext.h` via `LACredentialType`, `setCredential`, and `isCredentialSet`.

| Header / area | API surface | Status | Notes |
| --- | --- | --- | --- |
| `LocalAuthentication.h` / `LABase.h` | Umbrella imports and base Objective-C scaffolding | ✅ | Covered by the Swift bridge and module split; no standalone safe Rust type is required. |
| `LAContext.h` / `LAPolicy` | `LAContext` creation, invalidation, policy preflight/evaluation, localized titles/reason, reuse duration, `interactionNotAllowed`, `biometryType`, `evaluatedPolicyDomainState`, `domainState`, `LACredentialType`, `setCredential`, `isCredentialSet`, `LAAccessControlOperation`, and `LATouchIDAuthenticationMaximumAllowableReuseDuration` | ✅ | Exposed through `LAContext`, `LAPolicy`, `LACredentialType`, `BiometryType`, `LACompanionType`, and the `LADomainState*` snapshot types. |
| `LAContext.h` | `evaluateAccessControl:operation:localizedReason:reply:` | 🟡 | Exposed as `unsafe fn evaluate_access_control_raw(...)` over a borrowed `SecAccessControlRef`; a higher-level safe Security wrapper is not part of this crate yet. |
| `LAContext.h` | `maxBiometryFailures` | ⏭️ | Deprecated no-op property. |
| `LAContext.h` | `LAPolicyDeviceOwnerAuthenticationWithWristDetection` | ⏭️ | watchOS-only policy. |
| `LAError.h` | `LAErrorDomain` and current macOS `LAError` cases | ✅ | Surfaced via `LA_ERROR_DOMAIN`, `LAError`, and `code` / `message` / `domain` helpers. |
| `LAError.h` | Deprecated `TouchID*` and `WatchNotAvailable` aliases | ⏭️ | Deprecated aliases are intentionally omitted; numeric codes still round-trip through `LAError::code()`. |
| `LABiometryType.h` / `LACompanionType.h` / `LADomainState.h` | Biometry, companion, and domain-state snapshots | ✅ | Implemented as `BiometryType`, `LACompanionType`, `LADomainState`, `LADomainStateBiometry`, and `LADomainStateCompanion`. |
| `LARequirement.h` | `LAAuthenticationRequirement` and `LABiometryFallbackRequirement` builders | ✅ | Default, biometry, current-set, and biometry-with-fallback flows are wrapped. |
| `LARight.h` | `LARightState`, `LARight` state/tag/init/authorize/preflight/deauthorize | ✅ | Fully surfaced in Rust and the Swift bridge. |
| `LARightStore.h` | Shared store, fetch, save, save-with-secret, remove, remove-by-identifier, remove-all | ✅ | Implemented as `LARightStore`. |
| `LAPersistedRight.h` / `LASecret.h` | Persisted right, managed secret loading | ✅ | Implemented as `LAPersistedRight` and `LASecret`. |
| `LAPrivateKey.h` | `publicKey`, sign/decrypt capability checks, sign/decrypt operations, key-exchange capability checks, and `exchangeKeysWithPublicKey:secKeyAlgorithm:secKeyParameters:completion:` | ✅ | Implemented as `LAPrivateKey`, `SecKeyAlgorithm`, and `SecKeyExchangeParameters`. |
| `LAPublicKey.h` | Export, encrypt capability/operation, verify capability/operation | ✅ | Implemented as `LAPublicKey`. |
| `LAPublicDefines.h` | Raw `kLA*` constant families | 🟡 | Represented through typed Rust enums and helpers instead of one constant per SDK symbol. |
| `LAEnvironment.h` / `LAEnvironmentState.h` / `LAEnvironmentMechanism*.h` | `LAEnvironment`, observer callbacks, environment-state snapshots, and mechanism subclasses | ✅ | Implemented as `LAEnvironment`, `LAEnvironmentObserver`, `LAEnvironmentState`, `LAEnvironmentMechanism`, and the biometry/companion/user-password mechanism wrappers. |
