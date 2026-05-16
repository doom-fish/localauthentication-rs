use localauthentication::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = LAContext::new()?;
    context.set_interaction_not_allowed(true)?;
    context.set_localized_fallback_title(Some("Use Password"))?;
    context.set_localized_cancel_title(Some("Cancel"))?;
    context.set_localized_reason("inspect the device owner's authentication state")?;
    context.set_touch_id_authentication_allowable_reuse_duration(30.0)?;

    let credential = LACredential::application_password(b"secret".to_vec());
    assert!(context.set_credential(&credential)?);
    assert!(context.is_credential_set(LACredentialType::ApplicationPassword)?);
    assert!(context.clear_credential(LACredentialType::ApplicationPassword)?);

    let preflight =
        match context.can_evaluate_policy(LAPolicy::DeviceOwnerAuthenticationWithBiometrics) {
            Ok(true) => "biometry available".to_owned(),
            Ok(false) => "biometry unavailable without a framework error".to_owned(),
            Err(error) => format!("biometry unavailable: {error}"),
        };
    let domain_state = context.domain_state()?;

    println!("preflight: {preflight}");
    println!("localized reason: {}", context.localized_reason()?);
    println!("biometry type: {:?}", context.biometry_type()?);
    println!(
        "domain state hash bytes: {}",
        domain_state.state_hash().map_or(0, <[u8]>::len)
    );
    println!(
        "reuse max seconds: {}",
        LAContext::touch_id_authentication_maximum_allowable_reuse_duration()
    );
    println!("✅ localauth context + credentials OK");
    Ok(())
}
