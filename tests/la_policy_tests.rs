use localauthentication::prelude::*;

#[test]
fn policy_raw_values_match_sdk_constants() {
    assert_eq!(
        LAPolicy::DeviceOwnerAuthenticationWithBiometrics.raw_value(),
        1
    );
    assert_eq!(LAPolicy::DeviceOwnerAuthentication.raw_value(), 2);
    assert_eq!(
        LAPolicy::DeviceOwnerAuthenticationWithCompanion.raw_value(),
        3
    );
    assert_eq!(
        LAPolicy::DeviceOwnerAuthenticationWithBiometricsOrCompanion.raw_value(),
        4
    );
}

#[test]
fn policies_are_accepted_by_context_preflight() -> Result<(), Box<dyn std::error::Error>> {
    let context = LAContext::new()?;
    for policy in [
        LAPolicy::DeviceOwnerAuthenticationWithBiometrics,
        LAPolicy::DeviceOwnerAuthentication,
        LAPolicy::DeviceOwnerAuthenticationWithCompanion,
        LAPolicy::DeviceOwnerAuthenticationWithBiometricsOrCompanion,
    ] {
        let _ = context.can_evaluate_policy(policy);
    }
    Ok(())
}
