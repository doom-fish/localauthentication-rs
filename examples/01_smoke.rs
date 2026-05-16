use localauthentication::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = LAContext::new()?;
    context.set_interaction_not_allowed(true)?;
    context.set_localized_fallback_title(Some("Use Password"))?;
    context.set_localized_cancel_title(Some("Cancel"))?;
    context.set_allowable_reuse_duration(30.0)?;

    let fallback = context.localized_fallback_title()?;
    let cancel = context.localized_cancel_title()?;
    assert_eq!(fallback.as_deref(), Some("Use Password"));
    assert_eq!(cancel.as_deref(), Some("Cancel"));
    assert!(context.interaction_not_allowed()?);
    assert!((context.allowable_reuse_duration()? - 30.0).abs() < f64::EPSILON);

    let preflight =
        match context.can_evaluate_policy(Policy::DeviceOwnerAuthenticationWithBiometrics) {
            Ok(true) => "biometry available".to_owned(),
            Ok(false) => "biometry unavailable without a framework error".to_owned(),
            Err(error) => format!("biometry unavailable: {error}"),
        };
    let biometry = context.biometry_type()?;
    let domain_state_len = context
        .evaluated_policy_domain_state()?
        .map_or(0, |state| state.len());

    println!("preflight: {preflight}");
    println!("biometry type: {biometry:?}");
    println!("domain state bytes: {domain_state_len}");
    println!("✅ localauth context + canEvaluatePolicy OK");
    Ok(())
}
