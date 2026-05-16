use localauthentication::ffi;
use localauthentication::prelude::*;

#[test]
fn error_domain_matches_sdk() {
    assert_eq!(LA_ERROR_DOMAIN, "com.apple.LocalAuthentication");
    assert_eq!(LAError::domain(), LA_ERROR_DOMAIN);
}

#[test]
fn error_mapping_exposes_code_and_message() {
    let error = LAError::from_code_message(ffi::la_error::BIOMETRY_LOCKOUT, "locked");
    assert_eq!(error.code(), ffi::la_error::BIOMETRY_LOCKOUT);
    assert_eq!(error.message(), "locked");
    assert!(matches!(error, LAError::BiometryLockout(_)));
}
