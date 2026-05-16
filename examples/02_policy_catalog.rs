use localauthentication::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = LAContext::new()?;
    let policies = [
        LAPolicy::DeviceOwnerAuthenticationWithBiometrics,
        LAPolicy::DeviceOwnerAuthentication,
        LAPolicy::DeviceOwnerAuthenticationWithCompanion,
        LAPolicy::DeviceOwnerAuthenticationWithBiometricsOrCompanion,
    ];

    for policy in policies {
        println!(
            "{} => {:?}",
            policy.description(),
            context.can_evaluate_policy(policy)
        );
    }

    println!("✅ policy catalogue OK");
    Ok(())
}
