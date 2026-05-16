use localauthentication::prelude::*;

#[test]
fn credential_construction_and_context_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let context = LAContext::new()?;
    let password = LACredential::application_password(b"p@ssword".to_vec());
    let pin = LACredential::smart_card_pin(b"123456".to_vec());

    assert_eq!(
        password.credential_type(),
        LACredentialType::ApplicationPassword
    );
    assert_eq!(pin.credential_type(), LACredentialType::SmartCardPin);
    assert!(context.set_credential(&password)?);
    assert!(context.is_credential_set(LACredentialType::ApplicationPassword)?);
    assert!(context.clear_credential(LACredentialType::ApplicationPassword)?);

    assert!(context.set_credential(&pin)?);
    assert!(context.is_credential_set(LACredentialType::SmartCardPin)?);
    assert!(context.clear_credential(LACredentialType::SmartCardPin)?);
    Ok(())
}
