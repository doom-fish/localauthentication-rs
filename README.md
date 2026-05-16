# localauthentication-rs

Safe Rust bindings for Apple's [LocalAuthentication](https://developer.apple.com/documentation/localauthentication) framework on macOS.

> **Status:** v0.2.0 expands coverage across `LAContext`, `LAPolicy`, `LAError`, `LACredential`, `LAAuthenticationRequirement`, `LARight`, `LARightStore`, `LAPersistedRight`, `LAPublicKey`, `LAPrivateKey`, and `LASecret`.

## Platform notes

- The Rust crate is macOS-focused and links the system `LocalAuthentication.framework`.
- The Swift bridge now targets **macOS 13+**.
- `LAContext::domain_state()` and companion-domain details are macOS 15+ APIs.
- Persisted-right and key APIs can require signing or entitlements; the examples and tests treat `OSStatus -34018` as an expected environment limitation.

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
- `LAPersistedRight`, `LASecret`, `LAPrivateKey`, `LAPublicKey`, and `SecKeyAlgorithm` helpers for persisted secrets and asymmetric-key operations
- Backward-compatible aliases for the v0.1.x surface (`Policy`, `LocalAuthenticationError`, `context`, and `error`)

## Examples

The crate ships numbered examples for every logical area:

- `01_smoke` — `LAContext`, policies, credentials, and domain state
- `02_policy_catalog` — policy availability across biometric and companion modes
- `03_error_codes` — `LAError` and domain/code mapping
- `04_credentials` — `LACredential` helpers
- `05_authentication_requirements` — requirement builders and `LARight` construction
- `06_rights` — right state/tag/preflight/deauthorize flow
- `07_right_store` — shared `LARightStore` persistence entry points
- `08_persisted_right` — `LAPersistedRight`, `LASecret`, and `LAPrivateKey`
- `09_public_key` — `LAPublicKey` export, verify, and encrypt capability checks

Run the full verification matrix with:

```bash
cargo clippy --all-targets -- -D warnings
cargo test
for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done
```

For the header-by-header audit, see [`COVERAGE.md`](COVERAGE.md).

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
