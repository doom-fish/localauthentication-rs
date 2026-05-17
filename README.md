# localauthentication-rs

Safe Rust bindings for Apple's [LocalAuthentication](https://developer.apple.com/documentation/localauthentication) framework on macOS.

> **Status:** v0.3.0 adds **async API support** (gated behind the `async` feature) for executor-agnostic policy and access control evaluation. The synchronous API remains at v0.2.1 coverage across `LAContext`, `LAPolicy`, `LAError`, `LACredential`, `LAAuthenticationRequirement`, `LARight`, `LARightStore`, `LAPersistedRight`, `LAPublicKey`, `LAPrivateKey`, `LASecret`, and the macOS 15 `LAEnvironment` observer/state surface.

## Platform notes

- The Rust crate is macOS-focused and links the system `LocalAuthentication.framework`.
- The Swift bridge now targets **macOS 13+**.
- `LAContext::domain_state()` plus the `LAEnvironment::{current_user, state, add_observer}` surface are macOS 15+ APIs.
- Persisted-right and key APIs can require signing or entitlements; the examples and tests treat `OSStatus -34018` as an expected environment limitation.
- `LAPrivateKey::exchange_keys_with_public_key` uses `SecKeyExchangeParameters` for the requested derived-key length and optional shared-info KDF context.

## Quick start

```rust,no_run
use localauthentication::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = LAContext::new()?;
    context.set_interaction_not_allowed(true)?;
    context.set_localized_reason("inspect local authentication state")?;

    match context.can_evaluate_policy(LAPolicy::DeviceOwnerAuthenticationWithBiometrics) {
        Ok(true) => {
            println!("biometry: {:?}", context.biometry_type()?);
            println!("reason: {}", context.localized_reason()?);
        }
        Ok(false) => println!("policy unavailable without a framework error"),
        Err(error) => println!("policy unavailable: {error}"),
    }

    Ok(())
}
```

## Highlights

- `LAContext` lifecycle, policy preflight/evaluation, credential injection, domain-state snapshots, and raw access-control evaluation
- `LAPolicy`, `LAError`, `LA_ERROR_DOMAIN`, `BiometryType`, and `LACompanionType`
- `LAAuthenticationRequirement` and `LABiometryFallbackRequirement` builders for rights
- `LARight` and `LARightStore` for in-memory and persisted authorization flows
- `LAPersistedRight`, `LASecret`, `LAPrivateKey`, `LAPublicKey`, `SecKeyAlgorithm`, and `SecKeyExchangeParameters` helpers for persisted secrets and asymmetric-key operations
- `LAEnvironment`, `LAEnvironmentObserver`, `LAEnvironmentState`, and the environment mechanism subclasses for macOS 15 environment snapshots and change notifications
- Backward-compatible aliases for the v0.1.x surface (`Policy`, `LocalAuthenticationError`, `context`, and `error`)
- **Async API** (gated behind `async` feature): executor-agnostic async wrappers for policy and access control evaluation via `AsyncContextExt` trait

## Async API

When the `async` feature is enabled, the `async_api` module provides executor-agnostic async methods for authentication:

```rust,no_run
use localauthentication::prelude::*;
use localauthentication::async_api::AsyncContextExt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = LAContext::new()?;
    let policy = LAPolicy::DeviceOwnerAuthenticationWithBiometrics;

    let success = pollster::block_on(async {
        context
            .evaluate_policy_async(policy, "Authenticate please")?
            .await
    })?;
    
    println!("Authentication result: {success}");
    Ok(())
}
```

The async API:
- Works with **any** async runtime (Tokio, async-std, smol, pollster, etc.)
- Uses callback-based Swift FFI for true async operations
- Provides `AsyncContextExt` trait with async variants of `evaluate_policy_async` and `evaluate_access_control_async`
- Returns futures that resolve to `Result<bool, LAError>`

See `examples/02_async_policy.rs` for a complete example.

## Examples

The crate ships numbered examples for every logical area:

- `01_smoke` — `LAContext`, policies, credentials, and domain state
- `02_async_policy` — async policy evaluation using `AsyncLAContext` (requires `async` feature)
- `03_policy_catalog` — policy availability across biometric and companion modes
- `04_error_codes` — `LAError` and domain/code mapping
- `05_credentials` — `LACredential` helpers
- `06_authentication_requirements` — requirement builders and `LARight` construction
- `07_rights` — right state/tag/preflight/deauthorize flow
- `08_right_store` — shared `LARightStore` persistence entry points
- `09_persisted_right` — `LAPersistedRight`, `LASecret`, and `LAPrivateKey`
- `10_public_key` — `LAPublicKey` export plus `LAPrivateKey` verify/encrypt/key-exchange capability checks
- `11_environment` — `LAEnvironment`, observer registration, and mechanism snapshots

Run the full verification matrix with:

```bash
cargo clippy --all-targets -- -D warnings
cargo test
for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done
```

For the header-by-header audit, see [`COVERAGE.md`](COVERAGE.md).

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
