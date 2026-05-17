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
