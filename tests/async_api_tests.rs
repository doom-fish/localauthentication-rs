#![cfg(feature = "async")]

use localauthentication::prelude::*;
use localauthentication::async_api::AsyncContextExt;

#[test]
fn test_evaluate_policy_empty_reason() {
    let context = LAContext::new().expect("Failed to create LAContext");
    let policy = LAPolicy::DeviceOwnerAuthenticationWithBiometrics;

    let result = context.evaluate_policy_async(policy, "");

    // Should reject empty reason
    assert!(result.is_err());
}

fn user_presence() -> AccessControl {
    AccessControl::create(
        AccessControlProtection::WhenUnlockedThisDeviceOnly,
        AccessControlFlags::USER_PRESENCE,
    )
    .expect("create access control")
}

#[test]
fn test_evaluate_access_control_empty_reason() {
    let context = LAContext::new().expect("Failed to create LAContext");
    let access_control = user_presence();

    let result = context.evaluate_access_control_async(
        &access_control,
        LAAccessControlOperation::UseItem,
        "",
    );

    // Should reject empty reason
    assert!(matches!(result, Err(LAError::InvalidArgument(_))));
}

#[test]
fn async_access_control_evaluation_matches_the_synchronous_error() {
    let context = LAContext::new().expect("Failed to create LAContext");
    context
        .set_interaction_not_allowed(true)
        .expect("disable interaction");
    let access_control = user_presence();

    let sync_error = context
        .evaluate_access_control(
            &access_control,
            LAAccessControlOperation::UseItem,
            "Authenticate",
        )
        .expect_err("user presence cannot be satisfied without interaction");
    let pending = context
        .evaluate_access_control_async(
            &access_control,
            LAAccessControlOperation::UseItem,
            "Authenticate",
        )
        .expect("start evaluation");
    drop(access_control);
    let async_error = pollster::block_on(pending)
        .expect_err("user presence cannot be satisfied without interaction");

    assert!(
        matches!(async_error, LAError::NotInteractive(_)),
        "{async_error:?}"
    );
    assert_eq!(sync_error.code(), async_error.code());
}

#[test]
fn async_errors_keep_their_la_error_code() {
    let context = LAContext::new().expect("Failed to create LAContext");
    context
        .set_interaction_not_allowed(true)
        .expect("disable interaction");

    let error = pollster::block_on(
        context
            .evaluate_policy_async(LAPolicy::DeviceOwnerAuthentication, "Authenticate")
            .expect("start evaluation"),
    )
    .expect_err("a non-interactive evaluation cannot succeed");

    assert!(!matches!(error, LAError::BridgeFailed(_)), "{error:?}");
    assert!(
        (-10_000..0).contains(&error.code()),
        "expected an LAErrorDomain code, got {error:?}"
    );
}

#[test]
fn async_errors_match_the_synchronous_errors() {
    let context = LAContext::new().expect("Failed to create LAContext");
    context
        .set_interaction_not_allowed(true)
        .expect("disable interaction");

    let sync_error = context
        .evaluate_policy(LAPolicy::DeviceOwnerAuthentication, "Authenticate")
        .expect_err("a non-interactive evaluation cannot succeed");
    let async_error = pollster::block_on(
        context
            .evaluate_policy_async(LAPolicy::DeviceOwnerAuthentication, "Authenticate")
            .expect("start evaluation"),
    )
    .expect_err("a non-interactive evaluation cannot succeed");

    assert_eq!(sync_error.code(), async_error.code());
}

#[test]
fn nul_in_the_reason_is_rejected_before_starting() {
    let context = LAContext::new().expect("Failed to create LAContext");
    let error = context
        .evaluate_policy_async(LAPolicy::DeviceOwnerAuthentication, "bad\0reason")
        .expect_err("NUL bytes are rejected");
    assert!(matches!(error, LAError::InvalidArgument(_)), "{error:?}");
}
