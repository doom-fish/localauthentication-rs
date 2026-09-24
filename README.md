# localauthentication-rs

Safe Rust bindings for Apple's [LocalAuthentication](https://developer.apple.com/documentation/localauthentication) framework on macOS.

## Installation

```toml
[dependencies]
apple-localauthentication = "0.4"
```

The library is imported as `localauthentication`. The optional `async` feature adds executor-agnostic futures for policy and access-control evaluation.

## A successful evaluation is not a secure gate

`LAContext::evaluate_policy` returns `Ok(true)` when the user authenticated, but that boolean lives in your process: a debugger or a patched binary can flip it, so gating a secret on `if evaluate_policy(..)? { .. }` does not protect the secret. Let the system enforce the check instead:

- Keep the secret in a persisted right: `LARightStore::save_right_with_secret` stores it, and `LAPersistedRight::secret()` / `LASecret::load_data` only return it after the right's authorization requirement has been met. `LAPrivateKey` operations are bound to the right in the same way.
- Or store it as a keychain item protected by an access control, and let the keychain check it. With `security-rs`, create an `AccessControl` (for example `AccessControlFlags::USER_PRESENCE` or `BIOMETRY_CURRENT_SET`, re-exported by this crate), attach it with `KeychainOptions::access_control`, and pass the context to the query with `KeychainOptions::authentication_context(context.as_raw_la_context())` (an `unsafe fn`: the pointer must be a live `LAContext`), which sets `kSecUseAuthenticationContext`. `LAContext::evaluate_access_control` evaluates the same `AccessControl` ahead of time, so the keychain query does not prompt again. Keep the Rust `LAContext` alive for as long as the query uses it: dropping it invalidates the context.

```rust,no_run
use localauthentication::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let presence = AccessControl::create(
        AccessControlProtection::WhenUnlockedThisDeviceOnly,
        AccessControlFlags::USER_PRESENCE,
    )?;
    let context = LAContext::new()?;
    let evaluated = context.evaluate_access_control(
        &presence,
        LAAccessControlOperation::UseItem,
        "unlock the saved token",
    )?;
    println!("evaluated: {evaluated}");
    Ok(())
}
```

## Platform notes

- The Rust crate is macOS-focused and links the system `LocalAuthentication.framework`.
- Requires **macOS 13** or newer (the Swift bridge's deployment target).
- `LAContext::domain_state()` plus the `LAEnvironment::{current_user, state, add_observer}` surface are macOS 15+ APIs.
- Persisted-right and key APIs can require signing or entitlements; the examples and tests treat `OSStatus -34018` as an expected environment limitation.
- `LAPrivateKey::exchange_keys_with_public_key` uses `SecKeyExchangeParameters` for the requested derived-key length and optional shared-info KDF context.

## Timeouts, cancellation and dropping

- The synchronous calls that wait for the framework (`LAContext::evaluate_policy`, `evaluate_access_control`, `LARight` / `LAPersistedRight` authorization, the `LARightStore` operations, and the key and secret operations) wait at most `sync_timeout()`, 30 seconds by default. Change it for the whole process with `set_sync_timeout(Some(duration))`, or pass `None` to wait indefinitely. The async API never times out.
- A timed-out call returns `LAError::TimedOut` and is cancelled: an evaluation invalidates its `LAContext` (which dismisses the prompt), an authorization that completes later is deauthorized again, and a right that is saved after the timeout is removed again. Removals cannot be undone and may still complete after a timeout.
- Dropping an `LAContext` invalidates it, so a prompt that is still on screen is dismissed and any pending evaluation (including an async one) fails with `LAError::AppCancel`.

## Secrets and errors

- `LACredential` keeps its bytes in `zeroize::Zeroizing`, prints them as `<redacted>` in `Debug`, and compares them in constant time. `LASecret::load_data`, `LAPrivateKey::decrypt` and `LAPrivateKey::exchange_keys_with_public_key` return `Zeroizing<Vec<u8>>`, and the bridge wipes its copy. Copies held by the framework cannot be wiped from Rust.
- Errors from `LAErrorDomain` map to the typed `LAError` variants, from both the synchronous and the async API. Errors from other domains (for example `NSOSStatusErrorDomain -34018` when an entitlement is missing) become `LAError::Other`, with the domain and code in the message.

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

- `LAContext` lifecycle, policy preflight/evaluation, credential injection, domain-state snapshots, and access-control evaluation with `security-rs`'s `AccessControl` (re-exported with `AccessControlFlags` and `AccessControlProtection`)
- `LAPolicy`, `LAError`, `LA_ERROR_DOMAIN`, `BiometryType`, and `LACompanionType`
- `LAAuthenticationRequirement` and `LABiometryFallbackRequirement` builders for rights
- `LARight` and `LARightStore` for in-memory and persisted authorization flows
- `LAPersistedRight`, `LASecret`, `LAPrivateKey`, `LAPublicKey`, `SecKeyAlgorithm`, and `SecKeyExchangeParameters` helpers for persisted secrets and asymmetric-key operations
- `LAEnvironment`, `LAEnvironmentObserver`, `LAEnvironmentState`, and the environment mechanism subclasses for macOS 15 environment snapshots and change notifications
- Backward-compatible aliases for the v0.1.x surface (`Policy`, `LocalAuthenticationError`, `context`, and `error`)
- **Async API** (gated behind `async` feature): executor-agnostic async wrappers for policy and access control evaluation via `AsyncContextExt` trait

## Async API

When the `async` feature is enabled, the `async_api` module provides executor-agnostic async methods for authentication:

```rust,ignore
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
- Returns futures that resolve to `Result<bool, LAError>` with the same error variants as the synchronous API

See `examples/02_async_policy.rs` for a complete example.

## Examples

The crate ships numbered examples for every logical area:

- `01_smoke` — `LAContext`, policies, credentials, and domain state
- `02_async_policy` — async policy evaluation using `AsyncContextExt` (requires the `async` feature; shows an authentication prompt)
- `02_policy_catalog` — policy availability across biometric and companion modes
- `03_error_codes` — `LAError` and domain/code mapping
- `04_credentials` — `LACredential` helpers
- `05_authentication_requirements` — requirement builders and `LARight` construction
- `06_rights` — right state/tag/preflight/deauthorize flow
- `07_right_store` — shared `LARightStore` persistence entry points
- `08_persisted_right` — `LAPersistedRight`, `LASecret`, and `LAPrivateKey`
- `09_public_key` — `LAPublicKey` export plus `LAPrivateKey` verify/encrypt/key-exchange capability checks
- `10_environment` — `LAEnvironment`, observer registration, and mechanism snapshots

Run the full verification matrix with:

```bash
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
for ex in examples/*.rs; do name="$(basename "$ex" .rs)"; [ "$name" = 02_async_policy ] || cargo run --example "$name"; done
```

For the header-by-header audit, see [`COVERAGE.md`](COVERAGE.md).

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
