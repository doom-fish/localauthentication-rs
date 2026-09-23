# Changelog

All notable changes to `apple-localauthentication` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - Unreleased

### Security

- `LACredential` no longer derives `Debug`, `Clone` and `PartialEq` over the
  raw password or PIN. Its bytes live in `Zeroizing<Vec<u8>>`, `Debug` prints
  them as `<redacted>`, and equality is constant-time.
- A synchronous call that timed out left the Swift task running: the prompt
  stayed up, a later success was dropped, and `LARightStore::save_right*`
  could still persist a right after Rust had seen `TimedOut`. Timed-out calls
  are now cancelled; an evaluation invalidates its `LAContext`, an
  authorization that completes later is deauthorized, and a right saved after
  the timeout is removed again.
- The README now explains that a successful `evaluate_policy` is not a secure
  gate and how to bind secrets to the system check (persisted-right secrets
  and keys, or keychain access control with the evaluated context).
- `LASecret::load_data`, `LAPrivateKey::decrypt` and
  `LAPrivateKey::exchange_keys_with_public_key` return `Zeroizing<Vec<u8>>`,
  and the bridge wipes its copy before freeing it.

### Fixed

- Errors from domains other than `LAErrorDomain` were read as `LAError` codes
  (an `NSOSStatusErrorDomain -2` became `UserCancel`), codes outside `Int32`
  aborted the process, and an `NSError` with code 0 read as success. Foreign
  errors now become `LAError::Other` with the domain and code in the message.
- The async API reports the same typed `LAError` variants as the synchronous
  API instead of `BridgeFailed(String)` for every failure.
- `evaluate_policy_async` and `evaluate_access_control_async` leaked their
  completion when the reason contained a NUL byte.
- Dropping an `LAContext` invalidates it, so a prompt still on screen (after a
  timeout, or while an async evaluation is pending) is dismissed.
- Bridge handles are type-checked, so passing the wrong kind of handle returns
  `InvalidArgument` instead of confusing types.
- The `LARight` and `LAPersistedRight` state conversions no longer trap.
- The `02_async_policy` example requires the `async` feature, so building
  without features no longer fails.

### Changed

- **Breaking:** `LASecret::load_data`, `LAPrivateKey::decrypt` and
  `LAPrivateKey::exchange_keys_with_public_key` return `Zeroizing<Vec<u8>>`.
- **Breaking:** `LACredential` no longer implements a field-wise `Debug`;
  `PartialEq` compares the bytes in constant time.
- **Breaking:** a timed-out `evaluate_policy` or `evaluate_access_control_raw`
  invalidates the `LAContext`, and dropping an `LAContext` invalidates it.
- `doom-fish-utils` requirement is now `>=0.4.1, <0.5`; `rust-version` is now
  1.82.
- New dependency `zeroize` (`>=1.6, <1.9`; 1.9 needs Rust 1.85).

### Added

- `set_sync_timeout(Option<Duration>)` and `sync_timeout()` to configure how
  long the synchronous calls wait (default 30 s; `None` waits indefinitely).
- `LAContext::as_raw_la_context()`, the borrowed Objective-C `LAContext`, for
  keychain queries that use `kSecUseAuthenticationContext`.

## [0.3.5] - 2026-06-06

### Fixed

- Null-checked the localized reason in the async evaluations, guarded the
  environment-observer release trampoline, and removed the vestigial Swift
  bridge module map.

## [0.3.4] - 2026-05-20

- Widen `doom-fish-utils` dependency bound to `<0.4` so the 0.3.x SPSC-ring release resolves cleanly. No source changes.

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
