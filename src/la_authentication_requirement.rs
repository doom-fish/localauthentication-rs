//! `LAAuthenticationRequirement` and `LABiometryFallbackRequirement` wrappers.

use crate::ffi;
use crate::la_error::Result;
use crate::private::{bridge_ptr, OwnedHandle};

/// Authentication requirements that can be attached to a `LARight`.
#[derive(Debug)]
pub struct LAAuthenticationRequirement {
    handle: OwnedHandle,
}

impl LAAuthenticationRequirement {
    pub(crate) const fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }

    /// The framework's default requirement.
    ///
    /// # Errors
    ///
    /// Returns an error if the API is unavailable or the Swift bridge rejects the request.
    pub fn default_requirement() -> Result<Self> {
        let raw = bridge_ptr(|out, error_out| unsafe {
            ffi::la_authentication_requirement::la_authentication_requirement_default(
                out, error_out,
            )
        })?;
        Ok(Self {
            handle: OwnedHandle::new(
                raw,
                ffi::la_authentication_requirement::la_authentication_requirement_release,
            ),
        })
    }

    /// Require biometric authentication.
    ///
    /// # Errors
    ///
    /// Returns an error if the API is unavailable or the Swift bridge rejects the request.
    pub fn biometry_requirement() -> Result<Self> {
        let raw = bridge_ptr(|out, error_out| unsafe {
            ffi::la_authentication_requirement::la_authentication_requirement_biometry(
                out, error_out,
            )
        })?;
        Ok(Self {
            handle: OwnedHandle::new(
                raw,
                ffi::la_authentication_requirement::la_authentication_requirement_release,
            ),
        })
    }

    /// Require biometric authentication with the current enrolled set.
    ///
    /// # Errors
    ///
    /// Returns an error if the API is unavailable or the Swift bridge rejects the request.
    pub fn biometry_current_set_requirement() -> Result<Self> {
        let raw = bridge_ptr(|out, error_out| unsafe {
            ffi::la_authentication_requirement::la_authentication_requirement_biometry_current_set(
                out, error_out,
            )
        })?;
        Ok(Self {
            handle: OwnedHandle::new(
                raw,
                ffi::la_authentication_requirement::la_authentication_requirement_release,
            ),
        })
    }

    /// Require biometry with the supplied fallback requirement.
    ///
    /// # Errors
    ///
    /// Returns an error if the API is unavailable or the Swift bridge rejects the request.
    pub fn biometry_requirement_with_fallback(
        fallback: &LABiometryFallbackRequirement,
    ) -> Result<Self> {
        let raw = bridge_ptr(|out, error_out| unsafe {
            ffi::la_authentication_requirement::la_authentication_requirement_biometry_with_fallback(
                fallback.as_ptr(),
                out,
                error_out,
            )
        })?;
        Ok(Self {
            handle: OwnedHandle::new(
                raw,
                ffi::la_authentication_requirement::la_authentication_requirement_release,
            ),
        })
    }
}

/// Fallback requirements usable with `LAAuthenticationRequirement::biometry_requirement_with_fallback`.
#[derive(Debug)]
pub struct LABiometryFallbackRequirement {
    handle: OwnedHandle,
}

impl LABiometryFallbackRequirement {
    pub(crate) const fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }

    /// The framework's default fallback requirement.
    ///
    /// # Errors
    ///
    /// Returns an error if the API is unavailable or the Swift bridge rejects the request.
    pub fn default_requirement() -> Result<Self> {
        let raw = bridge_ptr(|out, error_out| unsafe {
            ffi::la_authentication_requirement::la_biometry_fallback_requirement_default(
                out, error_out,
            )
        })?;
        Ok(Self {
            handle: OwnedHandle::new(
                raw,
                ffi::la_authentication_requirement::la_biometry_fallback_requirement_release,
            ),
        })
    }

    /// Require the device passcode as the biometry fallback.
    ///
    /// # Errors
    ///
    /// Returns an error if the API is unavailable or the Swift bridge rejects the request.
    pub fn device_passcode_requirement() -> Result<Self> {
        let raw = bridge_ptr(|out, error_out| unsafe {
            ffi::la_authentication_requirement::la_biometry_fallback_requirement_device_passcode(
                out, error_out,
            )
        })?;
        Ok(Self {
            handle: OwnedHandle::new(
                raw,
                ffi::la_authentication_requirement::la_biometry_fallback_requirement_release,
            ),
        })
    }
}
