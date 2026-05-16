# Changelog

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
