//! `LAContext` and related enums.

use core::ffi::c_void;
use std::ptr;
use std::ptr::NonNull;

use crate::error::{LocalAuthenticationError, Result};
use crate::ffi;
use crate::private::{
    bridge_bool, bridge_f64, bridge_i32, bridge_opt_bytes, bridge_opt_string, bridge_ptr,
    bridge_unit, cstring, framework_bool_result,
};

/// Authentication policies supported by this crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Policy {
    DeviceOwnerAuthenticationWithBiometrics,
    DeviceOwnerAuthentication,
    DeviceOwnerAuthenticationWithCompanion,
    DeviceOwnerAuthenticationWithBiometricsOrCompanion,
}

impl Policy {
    pub(crate) const fn as_ffi(self) -> i32 {
        match self {
            Self::DeviceOwnerAuthenticationWithBiometrics => {
                ffi::policy::DEVICE_OWNER_AUTHENTICATION_WITH_BIOMETRICS
            }
            Self::DeviceOwnerAuthentication => ffi::policy::DEVICE_OWNER_AUTHENTICATION,
            Self::DeviceOwnerAuthenticationWithCompanion => {
                ffi::policy::DEVICE_OWNER_AUTHENTICATION_WITH_COMPANION
            }
            Self::DeviceOwnerAuthenticationWithBiometricsOrCompanion => {
                ffi::policy::DEVICE_OWNER_AUTHENTICATION_WITH_BIOMETRICS_OR_COMPANION
            }
        }
    }
}

/// Biometry kinds reported by `LAContext`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum BiometryType {
    None,
    TouchId,
    FaceId,
    OpticId,
    Unknown(i32),
}

impl BiometryType {
    #[must_use]
    pub const fn from_ffi(value: i32) -> Self {
        match value {
            ffi::biometry::NONE => Self::None,
            ffi::biometry::TOUCH_ID => Self::TouchId,
            ffi::biometry::FACE_ID => Self::FaceId,
            ffi::biometry::OPTIC_ID => Self::OpticId,
            other => Self::Unknown(other),
        }
    }
}

/// Managed wrapper around Apple's `LAContext`.
#[derive(Debug)]
pub struct LAContext {
    raw: NonNull<c_void>,
}

impl LAContext {
    /// Create a new authentication context.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge fails to allocate the underlying `LAContext`.
    pub fn new() -> Result<Self> {
        let raw = bridge_ptr(|out, error_out| unsafe { ffi::la_context_new(out, error_out) })?;
        Ok(Self { raw })
    }

    /// Invalidate this context.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn invalidate(&self) -> Result<()> {
        bridge_unit(|error_out| unsafe { ffi::la_context_invalidate(self.raw.as_ptr(), error_out) })
    }

    /// Check whether a policy can be evaluated without prompting the user.
    ///
    /// # Errors
    ///
    /// Returns a mapped framework error when the policy is unavailable, or a bridge error if the request itself fails.
    pub fn can_evaluate_policy(&self, policy: Policy) -> Result<bool> {
        let mut out_can_evaluate = 0_u8;
        let mut framework_error_code = 0_i32;
        let mut framework_error_message = ptr::null_mut();
        let mut bridge_error = ptr::null_mut();

        let status = unsafe {
            ffi::la_context_can_evaluate_policy(
                self.raw.as_ptr(),
                policy.as_ffi(),
                &mut out_can_evaluate,
                &mut framework_error_code,
                &mut framework_error_message,
                &mut bridge_error,
            )
        };
        if status != ffi::status::OK {
            return Err(crate::error::from_status(status, bridge_error));
        }

        framework_bool_result(
            out_can_evaluate != 0,
            framework_error_code,
            framework_error_message,
        )
    }

    /// Evaluate a policy with the supplied localized reason string.
    ///
    /// # Errors
    ///
    /// Returns a mapped framework error when evaluation fails, or a bridge error if the request itself fails.
    pub fn evaluate_policy(&self, policy: Policy, localized_reason: &str) -> Result<bool> {
        if localized_reason.is_empty() {
            return Err(LocalAuthenticationError::InvalidArgument(
                "localized reason must not be empty".to_owned(),
            ));
        }

        let localized_reason = cstring(localized_reason)?;
        let mut out_success = 0_u8;
        let mut framework_error_code = 0_i32;
        let mut framework_error_message = ptr::null_mut();
        let mut bridge_error = ptr::null_mut();

        let status = unsafe {
            ffi::la_context_evaluate_policy(
                self.raw.as_ptr(),
                policy.as_ffi(),
                localized_reason.as_ptr(),
                &mut out_success,
                &mut framework_error_code,
                &mut framework_error_message,
                &mut bridge_error,
            )
        };
        if status != ffi::status::OK {
            return Err(crate::error::from_status(status, bridge_error));
        }

        framework_bool_result(
            out_success != 0,
            framework_error_code,
            framework_error_message,
        )
    }

    /// Read the localized fallback title.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn localized_fallback_title(&self) -> Result<Option<String>> {
        bridge_opt_string(|out, error_out| unsafe {
            ffi::la_context_get_localized_fallback_title(self.raw.as_ptr(), out, error_out)
        })
    }

    /// Update the localized fallback title. Pass `None` to restore the default title.
    ///
    /// # Errors
    ///
    /// Returns an error if the title contains an interior NUL byte or the Swift bridge rejects the request.
    pub fn set_localized_fallback_title(&self, title: Option<&str>) -> Result<()> {
        let title = title.map(cstring).transpose()?;
        bridge_unit(|error_out| unsafe {
            ffi::la_context_set_localized_fallback_title(
                self.raw.as_ptr(),
                title.as_ref().map_or(ptr::null(), |value| value.as_ptr()),
                error_out,
            )
        })
    }

    /// Read the localized cancel title.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn localized_cancel_title(&self) -> Result<Option<String>> {
        bridge_opt_string(|out, error_out| unsafe {
            ffi::la_context_get_localized_cancel_title(self.raw.as_ptr(), out, error_out)
        })
    }

    /// Update the localized cancel title. Pass `None` to restore the default title.
    ///
    /// # Errors
    ///
    /// Returns an error if the title contains an interior NUL byte or the Swift bridge rejects the request.
    pub fn set_localized_cancel_title(&self, title: Option<&str>) -> Result<()> {
        let title = title.map(cstring).transpose()?;
        bridge_unit(|error_out| unsafe {
            ffi::la_context_set_localized_cancel_title(
                self.raw.as_ptr(),
                title.as_ref().map_or(ptr::null(), |value| value.as_ptr()),
                error_out,
            )
        })
    }

    /// Read the allowable biometric reuse duration, in seconds.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn allowable_reuse_duration(&self) -> Result<f64> {
        bridge_f64(|out, error_out| unsafe {
            ffi::la_context_get_allowable_reuse_duration(self.raw.as_ptr(), out, error_out)
        })
    }

    /// Update the allowable biometric reuse duration, in seconds.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is negative, non-finite, or the Swift bridge rejects the request.
    pub fn set_allowable_reuse_duration(&self, duration: f64) -> Result<()> {
        if !duration.is_finite() || duration < 0.0 {
            return Err(LocalAuthenticationError::InvalidArgument(
                "allowable reuse duration must be a finite, non-negative number".to_owned(),
            ));
        }

        bridge_unit(|error_out| unsafe {
            ffi::la_context_set_allowable_reuse_duration(self.raw.as_ptr(), duration, error_out)
        })
    }

    /// Read whether interactive authentication UI is disabled.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn interaction_not_allowed(&self) -> Result<bool> {
        bridge_bool(|out, error_out| unsafe {
            ffi::la_context_get_interaction_not_allowed(self.raw.as_ptr(), out, error_out)
        })
    }

    /// Enable or disable interactive authentication UI.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn set_interaction_not_allowed(&self, value: bool) -> Result<()> {
        bridge_unit(|error_out| unsafe {
            ffi::la_context_set_interaction_not_allowed(
                self.raw.as_ptr(),
                u8::from(value),
                error_out,
            )
        })
    }

    /// Read the currently reported biometry type.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn biometry_type(&self) -> Result<BiometryType> {
        let raw = bridge_i32(|out, error_out| unsafe {
            ffi::la_context_get_biometry_type(self.raw.as_ptr(), out, error_out)
        })?;
        Ok(BiometryType::from_ffi(raw))
    }

    /// Read the evaluated policy domain state bytes, if any are available.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn evaluated_policy_domain_state(&self) -> Result<Option<Vec<u8>>> {
        bridge_opt_bytes(|out, out_len, error_out| unsafe {
            ffi::la_context_get_evaluated_policy_domain_state(
                self.raw.as_ptr(),
                out,
                out_len,
                error_out,
            )
        })
    }
}

impl Drop for LAContext {
    fn drop(&mut self) {
        unsafe { ffi::la_context_release(self.raw.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::{LAContext, Result};

    #[test]
    fn property_round_trip_without_prompt() -> Result<()> {
        let context = LAContext::new()?;
        context.set_interaction_not_allowed(true)?;
        context.set_localized_fallback_title(Some("Use Password"))?;
        context.set_localized_cancel_title(Some("Cancel"))?;
        context.set_allowable_reuse_duration(30.0)?;

        assert!(context.interaction_not_allowed()?);
        assert_eq!(
            context.localized_fallback_title()?.as_deref(),
            Some("Use Password")
        );
        assert_eq!(context.localized_cancel_title()?.as_deref(), Some("Cancel"));
        assert!((context.allowable_reuse_duration()? - 30.0).abs() < f64::EPSILON);
        Ok(())
    }
}
