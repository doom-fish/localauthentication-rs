# localauthentication-rs

Safe Rust bindings for Apple's [LocalAuthentication](https://developer.apple.com/documentation/localauthentication) framework on macOS.

> **Status:** v0.1.0 covers the `LAContext` surface doom-fish crates need first: policy preflight and evaluation, biometry inspection, domain-state access, and common prompt-related properties.

## Quick start

```rust,no_run
use localauthentication::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = LAContext::new()?;
    context.set_interaction_not_allowed(true)?;

    match context.can_evaluate_policy(Policy::DeviceOwnerAuthenticationWithBiometrics) {
        Ok(true) => println!("biometry available: {:?}", context.biometry_type()?),
        Ok(false) => println!("policy unavailable without a framework error"),
        Err(error) => println!("policy unavailable: {error}"),
    }

    Ok(())
}
```

## Highlights

- `LAContext` lifecycle management with automatic release on drop
- `Policy` coverage for biometric, password, and companion-device authentication modes
- `can_evaluate_policy` and synchronous `evaluate_policy`
- `interaction_not_allowed`, localized fallback/cancel titles, and reuse-duration properties
- `biometry_type` and `evaluated_policy_domain_state`
- `LocalAuthenticationError` variants for common `LAError` cases

## Smoke example

Run the end-to-end smoke test with:

```bash
cargo run --all-features --example 01_smoke
```

The smoke test intentionally avoids triggering a biometric prompt. It exercises context creation, property round-trips, policy preflight, and biometry inspection only.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
