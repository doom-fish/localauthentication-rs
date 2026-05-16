//! Errors produced by the `LocalAuthentication` bridge.

use core::ffi::c_char;
use core::fmt;

use libc::free;

use crate::ffi;

/// Convenient result alias used throughout this crate.
pub type Result<T, E = LocalAuthenticationError> = std::result::Result<T, E>;

/// Top-level error type returned by this crate.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum LocalAuthenticationError {
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

impl LocalAuthenticationError {
    /// Numeric status or `LAError` code reported by the bridge.
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
}

impl fmt::Display for LocalAuthenticationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (code {})", self.message(), self.code())
    }
}

impl std::error::Error for LocalAuthenticationError {}

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

/// Build a `LocalAuthenticationError` from a status code and optional message.
pub(crate) fn from_status(status: i32, error_str: *mut c_char) -> LocalAuthenticationError {
    let message = take_owned_c_string(error_str);
    from_status_message(status, message)
}

/// Build a `LocalAuthenticationError` from a status code and message generated in Rust.
#[must_use]
pub const fn from_status_message(status: i32, message: String) -> LocalAuthenticationError {
    match status {
        ffi::status::INVALID_ARGUMENT => LocalAuthenticationError::InvalidArgument(message),
        ffi::status::TIMED_OUT => LocalAuthenticationError::TimedOut(message),
        ffi::status::BRIDGE_FAILED => LocalAuthenticationError::BridgeFailed(message),
        ffi::la_error::AUTHENTICATION_FAILED => {
            LocalAuthenticationError::AuthenticationFailed(message)
        }
        ffi::la_error::USER_CANCEL => LocalAuthenticationError::UserCancel(message),
        ffi::la_error::USER_FALLBACK => LocalAuthenticationError::UserFallback(message),
        ffi::la_error::SYSTEM_CANCEL => LocalAuthenticationError::SystemCancel(message),
        ffi::la_error::PASSCODE_NOT_SET => LocalAuthenticationError::PasscodeNotSet(message),
        ffi::la_error::BIOMETRY_NOT_AVAILABLE => {
            LocalAuthenticationError::BiometryNotAvailable(message)
        }
        ffi::la_error::BIOMETRY_NOT_ENROLLED => {
            LocalAuthenticationError::BiometryNotEnrolled(message)
        }
        ffi::la_error::BIOMETRY_LOCKOUT => LocalAuthenticationError::BiometryLockout(message),
        ffi::la_error::APP_CANCEL => LocalAuthenticationError::AppCancel(message),
        ffi::la_error::INVALID_CONTEXT => LocalAuthenticationError::InvalidContext(message),
        ffi::la_error::NOT_INTERACTIVE => LocalAuthenticationError::NotInteractive(message),
        ffi::la_error::COMPANION_NOT_AVAILABLE => {
            LocalAuthenticationError::CompanionNotAvailable(message)
        }
        ffi::la_error::BIOMETRY_NOT_PAIRED => LocalAuthenticationError::BiometryNotPaired(message),
        ffi::la_error::BIOMETRY_DISCONNECTED => {
            LocalAuthenticationError::BiometryDisconnected(message)
        }
        ffi::la_error::INVALID_DIMENSIONS => LocalAuthenticationError::InvalidDimensions(message),
        code => LocalAuthenticationError::Other { code, message },
    }
}

#[cfg(test)]
mod tests {
    use super::{from_status_message, LocalAuthenticationError};
    use crate::ffi;

    #[test]
    fn maps_common_la_error_codes() {
        let error = from_status_message(
            ffi::la_error::BIOMETRY_LOCKOUT,
            "biometry is locked".to_owned(),
        );
        assert!(matches!(
            error,
            LocalAuthenticationError::BiometryLockout(message)
            if message == "biometry is locked"
        ));
    }

    #[test]
    fn maps_bridge_status_codes() {
        let error = from_status_message(ffi::status::TIMED_OUT, "operation timed out".to_owned());
        assert!(matches!(
            error,
            LocalAuthenticationError::TimedOut(message)
            if message == "operation timed out"
        ));
    }
}
