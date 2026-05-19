# Changelog

## [0.3.3] - 2026-05-19

- Bump MSRV from 1.70 to 1.76 to match fleet baseline.

## [0.3.2] - 2026-05-18

- Widen doom-fish-utils version bound to `<0.3` so 0.2.x resolves.

## [0.3.1] - 2026-05-17

### Fixed

- Added panic-safety guards to async FFI callbacks (`evaluate_policy_callback`, `evaluate_access_control_callback`) using `doom_fish_utils::panic_safe::catch_user_panic` to prevent panics from unwinding across the FFI boundary.

## [0.3.0] - 2026-05-17

### Added

- **Async API** gated behind the `async` feature.
  - `AsyncLAContext` wrapper providing async methods for policy and access control evaluation.
  - `AsyncPolicyEvaluationFuture` and `AsyncAccessControlEvaluationFuture` for awaiting authentication results.
  - Callback-based Swift FFI thunks (`la_context_evaluate_policy_async`, `la_context_evaluate_access_control_async`).
  - Integration with `doom-fish-utils::completion` for executor-agnostic, runtime-agnostic async operations.
  - Example `examples/02_async_policy.rs` demonstrating async biometric authentication.
  - Test suite `tests/async_api_tests.rs` covering happy path and error cases.

### Changed

- Updated `Cargo.toml` to include `async` feature gate and `doom-fish-utils` + `pollster` dependencies.
- Updated `src/lib.rs` to conditionally export the new `async_api` module.

## [0.2.1] - 2026-05-16

### Added

- Wrapped macOS 15 `LAEnvironment`, `LAEnvironmentObserver`, `LAEnvironmentState`, and the biometry, companion, and user-password mechanism subclasses with observer registration support.
- Added `SecKeyExchangeParameters` and `LAPrivateKey::exchange_keys_with_public_key` for safe Diffie-Hellman-style key exchange.
- Added `tests/la_environment_tests.rs`, extended the key tests for live key exchange, and shipped `examples/10_environment.rs` plus an updated `09_public_key` example.

### Changed

- Completed the `COVERAGE_AUDIT.md` gap list and refreshed the README/COVERAGE docs for the fully wrapped environment and key-exchange surface.

## [0.2.0] - 2026-05-16

### Added

- Safe Rust wrappers for `LACredential`, `LAAuthenticationRequirement`, `LABiometryFallbackRequirement`, `LARight`, `LARightStore`, `LAPersistedRight`, `LASecret`, `LAPrivateKey`, `LAPublicKey`, and `SecKeyAlgorithm`.
- Expanded `LAContext` coverage for localized reason, credential APIs, companion-aware domain-state snapshots, and raw access-control evaluation.
- `LA_ERROR_DOMAIN` / `LAError` helpers together with backward-compatible `Policy` and `LocalAuthenticationError` aliases.
- Numbered examples `01_smoke` through `09_public_key` plus per-area integration tests.
- `COVERAGE.md` documenting the SDK audit and deferred framework areas.

### Changed

- Split the Swift bridge into per-area source files and raised the SwiftPM deployment target to macOS 13.
- Refreshed the README and crate metadata for the broader v0.2.0 surface.

## [0.1.0] - 2026-05-16

### Added

- `LAContext` wrapper with automatic release and manual `invalidate` support.
- Policy coverage for device-owner biometric, password, and companion-device authentication modes.
- Synchronous `can_evaluate_policy` and `evaluate_policy` APIs over the asynchronous framework callbacks.
- Property accessors for `interactionNotAllowed`, localized fallback/cancel titles, allowable reuse duration, `biometryType`, and `evaluatedPolicyDomainState`.
- `LocalAuthenticationError` mapping for common `LAError` values.
- SwiftPM bridge under `swift-bridge/` with `la_*` exports and a no-prompt smoke example `examples/01_smoke.rs`.
