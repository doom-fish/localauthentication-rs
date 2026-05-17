//! Async biometric authentication example
//!
//! This example demonstrates asynchronous policy evaluation using the async API.
//! It uses `pollster` to run async code synchronously for demonstration.

use localauthentication::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("LocalAuthentication Async Example");

    let context = LAContext::new()?;

    println!("Checking if biometric authentication is available...");
    let policy = LAPolicy::DeviceOwnerAuthenticationWithBiometrics;
    let can_evaluate = context.can_evaluate_policy(policy)?;

    if can_evaluate {
        println!("Biometric authentication is available!");
        println!("Starting async authentication...");

        #[cfg(feature = "async")]
        {
            use localauthentication::async_api::AsyncContextExt;

            let result = pollster::block_on(async {
                context
                    .evaluate_policy_async(policy, "Authenticate with biometrics")?
                    .await
            });

            match result {
                Ok(success) => {
                    println!("Authentication result: {success}");
                }
                Err(e) => {
                    println!("Authentication error: {e}");
                }
            }
        }

        #[cfg(not(feature = "async"))]
        {
            println!("Compile with --features async to run async example");
        }
    } else {
        println!("Biometric authentication is not available on this system");
    }

    Ok(())
}
