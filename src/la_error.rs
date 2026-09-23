//! Errors produced by the `LocalAuthentication` bridge.

use core::ffi::c_char;
use core::fmt;

use libc::free;

use crate::ffi;

/// `LocalAuthentication`'s `NSError` domain string.
pub const LA_ERROR_DOMAIN: &str = "com.apple.LocalAuthentication";

/// Convenient result alias used throughout this crate.
pub type Result<T, E = LAError> = std::result::Result<T, E>;

/// Top-level error type returned by this crate.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum LAError {
    /// Invalid input crossed the FFI boundary.
    InvalidArgument(String),
    /// The framework callback did not complete before the bridge timeout elapsed.
    TimedOut(String),
    /// The Swift bridge failed before reaching the framework call.
    BridgeFailed(String),
    /// Authentication was not successful because valid credentials were not provided.
    AuthenticationFailed(String),
    /// Authentication was cancelled by the user.
    UserCancel(String),
    /// Authentication was cancelled because the fallback button was tapped.
    UserFallback(String),
    /// Authentication was cancelled by the system.
    SystemCancel(String),
    /// Authentication cannot start because no device passcode is configured.
    PasscodeNotSet(String),
    /// Authentication cannot start because biometry is unavailable.
    BiometryNotAvailable(String),
    /// Authentication cannot start because no biometric identities are enrolled.
    BiometryNotEnrolled(String),
    /// Authentication cannot start because biometry is locked.
    BiometryLockout(String),
    /// Authentication was cancelled by the application.
    AppCancel(String),
    /// The `LAContext` has already been invalidated.
    InvalidContext(String),
    /// Authentication would require UI while interaction is disallowed.
    NotInteractive(String),
    /// Authentication cannot start because no companion device is nearby.
    CompanionNotAvailable(String),
    /// Authentication cannot start because the paired biometric accessory is unavailable.
    BiometryNotPaired(String),
    /// Authentication cannot start because the paired biometric accessory is disconnected.
    BiometryDisconnected(String),
    /// Authentication cannot start because an embedded UI size is invalid.
    InvalidDimensions(String),
    /// Catch-all for unmapped framework or bridge status codes.
    Other { code: i32, message: String },
}

/// Backward-compatible alias for the v0.1.x error type name.
pub type LocalAuthenticationError = LAError;

impl LAError {
    /// Numeric bridge, `LAError`, or `OSStatus` code reported by the bridge.
    #[must_use]
    pub const fn code(&self) -> i32 {
        match self {
            Self::InvalidArgument(_) => ffi::status::INVALID_ARGUMENT,
            Self::TimedOut(_) => ffi::status::TIMED_OUT,
            Self::BridgeFailed(_) => ffi::status::BRIDGE_FAILED,
            Self::AuthenticationFailed(_) => ffi::la_error::AUTHENTICATION_FAILED,
            Self::UserCancel(_) => ffi::la_error::USER_CANCEL,
            Self::UserFallback(_) => ffi::la_error::USER_FALLBACK,
            Self::SystemCancel(_) => ffi::la_error::SYSTEM_CANCEL,
            Self::PasscodeNotSet(_) => ffi::la_error::PASSCODE_NOT_SET,
            Self::BiometryNotAvailable(_) => ffi::la_error::BIOMETRY_NOT_AVAILABLE,
            Self::BiometryNotEnrolled(_) => ffi::la_error::BIOMETRY_NOT_ENROLLED,
            Self::BiometryLockout(_) => ffi::la_error::BIOMETRY_LOCKOUT,
            Self::AppCancel(_) => ffi::la_error::APP_CANCEL,
            Self::InvalidContext(_) => ffi::la_error::INVALID_CONTEXT,
            Self::NotInteractive(_) => ffi::la_error::NOT_INTERACTIVE,
            Self::CompanionNotAvailable(_) => ffi::la_error::COMPANION_NOT_AVAILABLE,
            Self::BiometryNotPaired(_) => ffi::la_error::BIOMETRY_NOT_PAIRED,
            Self::BiometryDisconnected(_) => ffi::la_error::BIOMETRY_DISCONNECTED,
            Self::InvalidDimensions(_) => ffi::la_error::INVALID_DIMENSIONS,
            Self::Other { code, .. } => *code,
        }
    }

    /// Human-readable description returned by the Swift bridge.
    #[must_use]
    pub fn message(&self) -> &str {
        match self {
            Self::InvalidArgument(message)
            | Self::TimedOut(message)
            | Self::BridgeFailed(message)
            | Self::AuthenticationFailed(message)
            | Self::UserCancel(message)
            | Self::UserFallback(message)
            | Self::SystemCancel(message)
            | Self::PasscodeNotSet(message)
            | Self::BiometryNotAvailable(message)
            | Self::BiometryNotEnrolled(message)
            | Self::BiometryLockout(message)
            | Self::AppCancel(message)
            | Self::InvalidContext(message)
            | Self::NotInteractive(message)
            | Self::CompanionNotAvailable(message)
            | Self::BiometryNotPaired(message)
            | Self::BiometryDisconnected(message)
            | Self::InvalidDimensions(message)
            | Self::Other { message, .. } => message,
        }
    }

    /// `LocalAuthentication`'s `NSError` domain string.
    #[must_use]
    pub const fn domain() -> &'static str {
        LA_ERROR_DOMAIN
    }

    /// Build an error value from a numeric code and message.
    #[must_use]
    pub fn from_code_message(code: i32, message: impl Into<String>) -> Self {
        from_status_message(code, message.into())
    }
}

impl fmt::Display for LAError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (code {})", self.message(), self.code())
    }
}

impl std::error::Error for LAError {}

/// Take ownership of a Swift-allocated C string and free it with `libc::free`.
pub(crate) fn take_owned_c_string(ptr: *mut c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }

    let string = unsafe { core::ffi::CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned();
    unsafe { free(ptr.cast()) };
    string
}

/// Take ownership of a Swift-allocated byte buffer and free it with `libc::free`.
pub(crate) fn take_owned_buffer(ptr: *mut u8, len: usize) -> Vec<u8> {
    if ptr.is_null() || len == 0 {
        if !ptr.is_null() {
            unsafe { free(ptr.cast()) };
        }
        return Vec::new();
    }

    let bytes = unsafe { std::slice::from_raw_parts(ptr, len) }.to_vec();
    unsafe { free(ptr.cast()) };
    bytes
}

/// Build an `LAError` from a status code and optional message.
pub(crate) fn from_status(status: i32, error_str: *mut c_char) -> LAError {
    let message = take_owned_c_string(error_str);
    from_status_message(status, message)
}

/// Build an `LAError` from a status code and message generated in Rust.
pub(crate) const fn from_status_message(status: i32, message: String) -> LAError {
    match status {
        ffi::status::INVALID_ARGUMENT => LAError::InvalidArgument(message),
        ffi::status::TIMED_OUT => LAError::TimedOut(message),
        ffi::status::BRIDGE_FAILED => LAError::BridgeFailed(message),
        ffi::la_error::AUTHENTICATION_FAILED => LAError::AuthenticationFailed(message),
        ffi::la_error::USER_CANCEL => LAError::UserCancel(message),
        ffi::la_error::USER_FALLBACK => LAError::UserFallback(message),
        ffi::la_error::SYSTEM_CANCEL => LAError::SystemCancel(message),
        ffi::la_error::PASSCODE_NOT_SET => LAError::PasscodeNotSet(message),
        ffi::la_error::BIOMETRY_NOT_AVAILABLE => LAError::BiometryNotAvailable(message),
        ffi::la_error::BIOMETRY_NOT_ENROLLED => LAError::BiometryNotEnrolled(message),
        ffi::la_error::BIOMETRY_LOCKOUT => LAError::BiometryLockout(message),
        ffi::la_error::APP_CANCEL => LAError::AppCancel(message),
        ffi::la_error::INVALID_CONTEXT => LAError::InvalidContext(message),
        ffi::la_error::NOT_INTERACTIVE => LAError::NotInteractive(message),
        ffi::la_error::COMPANION_NOT_AVAILABLE => LAError::CompanionNotAvailable(message),
        ffi::la_error::BIOMETRY_NOT_PAIRED => LAError::BiometryNotPaired(message),
        ffi::la_error::BIOMETRY_DISCONNECTED => LAError::BiometryDisconnected(message),
        ffi::la_error::INVALID_DIMENSIONS => LAError::InvalidDimensions(message),
        code => LAError::Other { code, message },
    }
}

#[cfg(test)]
mod tests {
    use core::ffi::c_char;
    use std::ffi::CString;
    use std::ptr;

    use super::{from_status, LAError, LA_ERROR_DOMAIN};
    use crate::ffi;

    unsafe extern "C" {
        fn la_bridge_status_for_error(
            domain: *const c_char,
            code: isize,
            error_out: *mut *mut c_char,
        ) -> i32;
    }

    fn bridged(domain: &str, code: isize) -> LAError {
        let domain = CString::new(domain).expect("domain");
        let mut error = ptr::null_mut();
        let status = unsafe { la_bridge_status_for_error(domain.as_ptr(), code, &raw mut error) };
        from_status(status, error)
    }

    #[test]
    fn la_domain_codes_map_to_typed_variants() {
        assert!(matches!(
            bridged(LA_ERROR_DOMAIN, -2),
            LAError::UserCancel(message) if message == "synthetic error"
        ));
        assert!(matches!(
            bridged(LA_ERROR_DOMAIN, -8),
            LAError::BiometryLockout(_)
        ));
        assert!(matches!(
            bridged(LA_ERROR_DOMAIN, -1004),
            LAError::NotInteractive(_)
        ));
    }

    #[test]
    fn foreign_domain_codes_never_enter_the_la_code_space() {
        let error = bridged("NSOSStatusErrorDomain", -2);
        assert_eq!(error.code(), ffi::status::UNKNOWN);
        assert!(matches!(error, LAError::Other { .. }), "{error:?}");
        assert!(
            error
                .message()
                .contains("NSOSStatusErrorDomain error -2: synthetic error"),
            "{error:?}"
        );

        let error = bridged("NSCocoaErrorDomain", isize::MAX);
        assert_eq!(error.code(), ffi::status::UNKNOWN);
        assert!(error.message().contains(&isize::MAX.to_string()));
    }

    #[test]
    fn unusable_la_domain_codes_become_unknown() {
        for code in [0, -10_000, -10_002, isize::MIN] {
            let error = bridged(LA_ERROR_DOMAIN, code);
            assert_eq!(error.code(), ffi::status::UNKNOWN, "{code}: {error:?}");
        }
    }

    #[test]
    fn maps_common_la_error_codes() {
        let error = LAError::from_code_message(
            ffi::la_error::BIOMETRY_LOCKOUT,
            "biometry is locked".to_owned(),
        );
        assert!(matches!(
            error,
            LAError::BiometryLockout(message)
            if message == "biometry is locked"
        ));
    }

    #[test]
    fn maps_bridge_status_codes() {
        let error =
            LAError::from_code_message(ffi::status::TIMED_OUT, "operation timed out".to_owned());
        assert!(matches!(
            error,
            LAError::TimedOut(message)
            if message == "operation timed out"
        ));
    }
}
