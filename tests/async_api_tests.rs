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

#[test]
fn test_evaluate_access_control_null_pointer() {
    let context = LAContext::new().expect("Failed to create LAContext");

    let result = unsafe {
        context.evaluate_access_control_async(
            std::ptr::null(),
            LAAccessControlOperation::UseItem,
            "Authenticate",
        )
    };

    // Should reject null pointer
    assert!(result.is_err());
}

#[test]
fn test_evaluate_access_control_empty_reason() {
    let context = LAContext::new().expect("Failed to create LAContext");

    let result = unsafe {
        context.evaluate_access_control_async(
            1 as *const std::ffi::c_void,
            LAAccessControlOperation::UseItem,
            "",
        )
    };

    // Should reject empty reason
    assert!(result.is_err());
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
