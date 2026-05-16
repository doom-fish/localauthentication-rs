# Changelog

## [0.1.0] - 2026-05-16

### Added

- `LAContext` wrapper with automatic release and manual `invalidate` support.
- Policy coverage for device-owner biometric, password, and companion-device authentication modes.
- Synchronous `can_evaluate_policy` and `evaluate_policy` APIs over the asynchronous framework callbacks.
- Property accessors for `interactionNotAllowed`, localized fallback/cancel titles, allowable reuse duration, `biometryType`, and `evaluatedPolicyDomainState`.
- `LocalAuthenticationError` mapping for common `LAError` values.
- SwiftPM bridge under `swift-bridge/` with `la_*` exports and a no-prompt smoke example `examples/01_smoke.rs`.
