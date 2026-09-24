use localauthentication::prelude::*;

#[test]
fn la_context_round_trips_properties() -> Result<(), Box<dyn std::error::Error>> {
    let context = LAContext::new()?;
    context.set_interaction_not_allowed(true)?;
    context.set_localized_fallback_title(Some("Use Password"))?;
    context.set_localized_cancel_title(Some("Cancel"))?;
    context.set_localized_reason("open the secure vault")?;
    context.set_touch_id_authentication_allowable_reuse_duration(15.0)?;

    assert!(context.interaction_not_allowed()?);
    assert_eq!(
        context.localized_fallback_title()?.as_deref(),
        Some("Use Password")
    );
    assert_eq!(context.localized_cancel_title()?.as_deref(), Some("Cancel"));
    assert_eq!(context.localized_reason()?, "open the secure vault");
    assert!(
        (context.touch_id_authentication_allowable_reuse_duration()? - 15.0).abs() < f64::EPSILON
    );
    assert!(LAContext::touch_id_authentication_maximum_allowable_reuse_duration() >= 300.0);

    let domain_state = context.domain_state()?;
    let _ = domain_state.biometry().biometry_type();
    let _ = domain_state.state_hash();
    if let Some(companion) = domain_state.companion() {
        for companion_type in companion.available_companion_types() {
            let _ = companion.state_hash_for(*companion_type);
        }
    }

    Ok(())
}

fn user_presence() -> Result<AccessControl, Box<dyn std::error::Error>> {
    Ok(AccessControl::create(
        AccessControlProtection::WhenUnlockedThisDeviceOnly,
        AccessControlFlags::USER_PRESENCE,
    )?)
}

#[test]
fn access_control_evaluation_checks_the_reason_before_starting(
) -> Result<(), Box<dyn std::error::Error>> {
    let context = LAContext::new()?;
    let access_control = user_presence()?;
    for reason in ["", "bad\0reason"] {
        let error = context
            .evaluate_access_control(&access_control, LAAccessControlOperation::UseItem, reason)
            .expect_err("the reason is rejected before evaluation starts");
        assert!(matches!(error, LAError::InvalidArgument(_)), "{error:?}");
    }
    Ok(())
}

#[test]
fn non_interactive_access_control_evaluation_fails_without_a_prompt(
) -> Result<(), Box<dyn std::error::Error>> {
    let context = LAContext::new()?;
    context.set_interaction_not_allowed(true)?;
    let access_control = user_presence()?;
    let error = context
        .evaluate_access_control(
            &access_control,
            LAAccessControlOperation::UseItem,
            "Authenticate",
        )
        .expect_err("user presence cannot be satisfied without interaction");
    assert!(matches!(error, LAError::NotInteractive(_)), "{error:?}");
    Ok(())
}
