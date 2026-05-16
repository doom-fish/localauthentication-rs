//! `LARight` wrappers.

use crate::ffi;
use crate::la_authentication_requirement::LAAuthenticationRequirement;
use crate::la_error::{LAError, Result};
use crate::private::{bridge_i32, bridge_i64, bridge_ptr, bridge_unit, cstring, OwnedHandle};

/// Possible authorization states for a `LARight`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LARightState {
    Unknown,
    Authorizing,
    Authorized,
    NotAuthorized,
    UnknownValue(i32),
}

impl LARightState {
    #[must_use]
    pub const fn from_ffi(value: i32) -> Self {
        match value {
            ffi::la_right::STATE_UNKNOWN => Self::Unknown,
            ffi::la_right::STATE_AUTHORIZING => Self::Authorizing,
            ffi::la_right::STATE_AUTHORIZED => Self::Authorized,
            ffi::la_right::STATE_NOT_AUTHORIZED => Self::NotAuthorized,
            other => Self::UnknownValue(other),
        }
    }
}

/// Managed wrapper around Apple's `LARight`.
#[derive(Debug)]
pub struct LARight {
    handle: OwnedHandle,
}

impl LARight {
    pub(crate) const fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }

    /// Create a right with the framework's default authentication requirement.
    ///
    /// # Errors
    ///
    /// Returns an error if the API is unavailable or the Swift bridge rejects the request.
    pub fn new() -> Result<Self> {
        let raw =
            bridge_ptr(|out, error_out| unsafe { ffi::la_right::la_right_new(out, error_out) })?;
        Ok(Self {
            handle: OwnedHandle::new(raw, ffi::la_right::la_right_release),
        })
    }

    /// Create a right with an explicit authentication requirement.
    ///
    /// # Errors
    ///
    /// Returns an error if the API is unavailable or the Swift bridge rejects the request.
    pub fn new_with_requirement(requirement: &LAAuthenticationRequirement) -> Result<Self> {
        let raw = bridge_ptr(|out, error_out| unsafe {
            ffi::la_right::la_right_new_with_requirement(requirement.as_ptr(), out, error_out)
        })?;
        Ok(Self {
            handle: OwnedHandle::new(raw, ffi::la_right::la_right_release),
        })
    }

    /// The current authorization state.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn state(&self) -> Result<LARightState> {
        let raw = bridge_i32(|out, error_out| unsafe {
            ffi::la_right::la_right_get_state(self.handle.as_ptr(), out, error_out)
        })?;
        Ok(LARightState::from_ffi(raw))
    }

    /// Application-controlled integer tag.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn tag(&self) -> Result<i64> {
        bridge_i64(|out, error_out| unsafe {
            ffi::la_right::la_right_get_tag(self.handle.as_ptr(), out, error_out)
        })
    }

    /// Update the application-controlled integer tag.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn set_tag(&self, tag: i64) -> Result<()> {
        bridge_unit(|error_out| unsafe {
            ffi::la_right::la_right_set_tag(self.handle.as_ptr(), tag, error_out)
        })
    }

    /// Attempt to authorize the right.
    ///
    /// # Errors
    ///
    /// Returns a mapped framework or bridge error when authorization fails.
    pub fn authorize(&self, localized_reason: &str) -> Result<()> {
        if localized_reason.is_empty() {
            return Err(LAError::InvalidArgument(
                "localized reason must not be empty".to_owned(),
            ));
        }
        let localized_reason = cstring(localized_reason)?;
        bridge_unit(|error_out| unsafe {
            ffi::la_right::la_right_authorize(
                self.handle.as_ptr(),
                localized_reason.as_ptr(),
                error_out,
            )
        })
    }

    /// Preflight whether the right can eventually be authorized.
    ///
    /// # Errors
    ///
    /// Returns a mapped framework or bridge error when authorization is not possible.
    pub fn check_can_authorize(&self) -> Result<()> {
        bridge_unit(|error_out| unsafe {
            ffi::la_right::la_right_check_can_authorize(self.handle.as_ptr(), error_out)
        })
    }

    /// Deauthorize the right.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn deauthorize(&self) -> Result<()> {
        bridge_unit(|error_out| unsafe {
            ffi::la_right::la_right_deauthorize(self.handle.as_ptr(), error_out)
        })
    }
}
