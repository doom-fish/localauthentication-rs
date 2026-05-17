//! `LAContext` and related `LocalAuthentication` value types.

use core::ffi::c_void;
use std::collections::BTreeMap;
use std::ptr;

use crate::ffi;
use crate::la_credential::{LACredential, LACredentialType};
use crate::la_error::{from_status, LAError, Result};
use crate::la_policy::LAPolicy;
use crate::private::{
    bridge_bool, bridge_f64, bridge_i32, bridge_i32_vec, bridge_opt_bytes, bridge_opt_string,
    bridge_ptr, bridge_string, bridge_unit, cstring, framework_bool_result, OwnedHandle,
};

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

/// Companion kinds reported by `LADomainState`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum LACompanionType {
    Watch,
    Mac,
    Vision,
    Unknown(i32),
}

impl LACompanionType {
    #[must_use]
    pub const fn from_ffi(value: i32) -> Self {
        match value {
            ffi::companion::WATCH => Self::Watch,
            ffi::companion::MAC => Self::Mac,
            ffi::companion::VISION => Self::Vision,
            other => Self::Unknown(other),
        }
    }

    #[must_use]
    pub const fn raw_value(self) -> i32 {
        match self {
            Self::Watch => ffi::companion::WATCH,
            Self::Mac => ffi::companion::MAC,
            Self::Vision => ffi::companion::VISION,
            Self::Unknown(value) => value,
        }
    }
}

/// Access-control operations supported by `LAContext::evaluate_access_control_raw`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LAAccessControlOperation {
    CreateItem,
    UseItem,
    CreateKey,
    UseKeySign,
    UseKeyDecrypt,
    UseKeyKeyExchange,
}

impl LAAccessControlOperation {
    #[must_use]
    pub const fn raw_value(self) -> i32 {
        match self {
            Self::CreateItem => ffi::la_context::ACCESS_CONTROL_OPERATION_CREATE_ITEM,
            Self::UseItem => ffi::la_context::ACCESS_CONTROL_OPERATION_USE_ITEM,
            Self::CreateKey => ffi::la_context::ACCESS_CONTROL_OPERATION_CREATE_KEY,
            Self::UseKeySign => ffi::la_context::ACCESS_CONTROL_OPERATION_USE_KEY_SIGN,
            Self::UseKeyDecrypt => ffi::la_context::ACCESS_CONTROL_OPERATION_USE_KEY_DECRYPT,
            Self::UseKeyKeyExchange => {
                ffi::la_context::ACCESS_CONTROL_OPERATION_USE_KEY_KEY_EXCHANGE
            }
        }
    }
}

/// A snapshot of `LAContext.domainState.biometry`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LADomainStateBiometry {
    biometry_type: BiometryType,
    state_hash: Option<Vec<u8>>,
}

impl LADomainStateBiometry {
    #[must_use]
    pub const fn biometry_type(&self) -> BiometryType {
        self.biometry_type
    }

    #[must_use]
    pub fn state_hash(&self) -> Option<&[u8]> {
        self.state_hash.as_deref()
    }
}

/// A snapshot of `LAContext.domainState.companion`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LADomainStateCompanion {
    available_companion_types: Vec<LACompanionType>,
    state_hash: Option<Vec<u8>>,
    per_type_state_hashes: BTreeMap<LACompanionType, Vec<u8>>,
}

impl LADomainStateCompanion {
    #[must_use]
    pub fn available_companion_types(&self) -> &[LACompanionType] {
        &self.available_companion_types
    }

    #[must_use]
    pub fn state_hash(&self) -> Option<&[u8]> {
        self.state_hash.as_deref()
    }

    #[must_use]
    pub fn state_hash_for(&self, companion_type: LACompanionType) -> Option<&[u8]> {
        self.per_type_state_hashes
            .get(&companion_type)
            .map(std::vec::Vec::as_slice)
    }
}

/// A snapshot of `LAContext.domainState`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LADomainState {
    state_hash: Option<Vec<u8>>,
    biometry: LADomainStateBiometry,
    companion: Option<LADomainStateCompanion>,
}

impl LADomainState {
    #[must_use]
    pub fn state_hash(&self) -> Option<&[u8]> {
        self.state_hash.as_deref()
    }

    #[must_use]
    pub const fn biometry(&self) -> &LADomainStateBiometry {
        &self.biometry
    }

    #[must_use]
    pub const fn companion(&self) -> Option<&LADomainStateCompanion> {
        self.companion.as_ref()
    }
}

/// Managed wrapper around Apple's `LAContext`.
#[derive(Debug)]
pub struct LAContext {
    handle: OwnedHandle,
}

impl LAContext {
    /// Create a new authentication context.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge fails to allocate the underlying `LAContext`.
    pub fn new() -> Result<Self> {
        let raw = bridge_ptr(|out, error_out| unsafe {
            ffi::la_context::la_context_new(out, error_out)
        })?;
        Ok(Self {
            handle: OwnedHandle::new(raw, ffi::la_context::la_context_release),
        })
    }

    /// Invalidate this context.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn invalidate(&self) -> Result<()> {
        bridge_unit(|error_out| unsafe {
            ffi::la_context::la_context_invalidate(self.handle.as_ptr(), error_out)
        })
    }

    /// Check whether a policy can be evaluated without prompting the user.
    ///
    /// # Errors
    ///
    /// Returns a mapped framework error when the policy is unavailable, or a bridge error if the request itself fails.
    pub fn can_evaluate_policy(&self, policy: LAPolicy) -> Result<bool> {
        let mut out_can_evaluate = 0_u8;
        let mut framework_error_code = 0_i32;
        let mut framework_error_message = ptr::null_mut();
        let mut bridge_error = ptr::null_mut();

        let status = unsafe {
            ffi::la_context::la_context_can_evaluate_policy(
                self.handle.as_ptr(),
                policy.as_ffi(),
                &mut out_can_evaluate,
                &mut framework_error_code,
                &mut framework_error_message,
                &mut bridge_error,
            )
        };
        if status != ffi::status::OK {
            return Err(from_status(status, bridge_error));
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
    /// Returns a mapped framework or bridge error when evaluation fails.
    pub fn evaluate_policy(&self, policy: LAPolicy, localized_reason: &str) -> Result<bool> {
        if localized_reason.is_empty() {
            return Err(LAError::InvalidArgument(
                "localized reason must not be empty".to_owned(),
            ));
        }

        let localized_reason = cstring(localized_reason)?;
        bridge_bool(|out, error_out| unsafe {
            ffi::la_context::la_context_evaluate_policy(
                self.handle.as_ptr(),
                policy.as_ffi(),
                localized_reason.as_ptr(),
                out,
                error_out,
            )
        })
    }

    /// Evaluate a `SecAccessControlRef` for the given operation.
    ///
    /// # Safety
    ///
    /// `access_control` must be a valid borrowed `SecAccessControlRef` for the duration of the call.
    ///
    /// # Errors
    ///
    /// Returns a mapped framework or bridge error when evaluation fails.
    pub unsafe fn evaluate_access_control_raw(
        &self,
        access_control: *const c_void,
        operation: LAAccessControlOperation,
        localized_reason: &str,
    ) -> Result<bool> {
        if access_control.is_null() {
            return Err(LAError::InvalidArgument(
                "access control pointer must not be null".to_owned(),
            ));
        }
        if localized_reason.is_empty() {
            return Err(LAError::InvalidArgument(
                "localized reason must not be empty".to_owned(),
            ));
        }

        let localized_reason = cstring(localized_reason)?;
        bridge_bool(|out, error_out| {
            ffi::la_context::la_context_evaluate_access_control(
                self.handle.as_ptr(),
                access_control,
                operation.raw_value(),
                localized_reason.as_ptr(),
                out,
                error_out,
            )
        })
    }

    /// Read the localized fallback title.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn localized_fallback_title(&self) -> Result<Option<String>> {
        bridge_opt_string(|out, error_out| unsafe {
            ffi::la_context::la_context_get_localized_fallback_title(
                self.handle.as_ptr(),
                out,
                error_out,
            )
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
            ffi::la_context::la_context_set_localized_fallback_title(
                self.handle.as_ptr(),
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
            ffi::la_context::la_context_get_localized_cancel_title(
                self.handle.as_ptr(),
                out,
                error_out,
            )
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
            ffi::la_context::la_context_set_localized_cancel_title(
                self.handle.as_ptr(),
                title.as_ref().map_or(ptr::null(), |value| value.as_ptr()),
                error_out,
            )
        })
    }

    /// Read the default localized reason used for authentication requests.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn localized_reason(&self) -> Result<String> {
        bridge_string(|out, error_out| unsafe {
            ffi::la_context::la_context_get_localized_reason(self.handle.as_ptr(), out, error_out)
        })
    }

    /// Update the default localized reason used for authentication requests.
    ///
    /// # Errors
    ///
    /// Returns an error if the string contains an interior NUL byte or the Swift bridge rejects the request.
    pub fn set_localized_reason(&self, localized_reason: &str) -> Result<()> {
        let localized_reason = cstring(localized_reason)?;
        bridge_unit(|error_out| unsafe {
            ffi::la_context::la_context_set_localized_reason(
                self.handle.as_ptr(),
                localized_reason.as_ptr(),
                error_out,
            )
        })
    }

    /// Read the allowable biometric reuse duration, in seconds.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn touch_id_authentication_allowable_reuse_duration(&self) -> Result<f64> {
        bridge_f64(|out, error_out| unsafe {
            ffi::la_context::la_context_get_touch_id_authentication_allowable_reuse_duration(
                self.handle.as_ptr(),
                out,
                error_out,
            )
        })
    }

    /// Backward-compatible alias for `touch_id_authentication_allowable_reuse_duration`.
    ///
    /// # Errors
    ///
    /// Propagates any error returned by `touch_id_authentication_allowable_reuse_duration`.
    pub fn allowable_reuse_duration(&self) -> Result<f64> {
        self.touch_id_authentication_allowable_reuse_duration()
    }

    /// Update the allowable biometric reuse duration, in seconds.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is negative, non-finite, or the Swift bridge rejects the request.
    pub fn set_touch_id_authentication_allowable_reuse_duration(
        &self,
        duration: f64,
    ) -> Result<()> {
        if !duration.is_finite() || duration < 0.0 {
            return Err(LAError::InvalidArgument(
                "allowable reuse duration must be a finite, non-negative number".to_owned(),
            ));
        }

        bridge_unit(|error_out| unsafe {
            ffi::la_context::la_context_set_touch_id_authentication_allowable_reuse_duration(
                self.handle.as_ptr(),
                duration,
                error_out,
            )
        })
    }

    /// Backward-compatible alias for `set_touch_id_authentication_allowable_reuse_duration`.
    ///
    /// # Errors
    ///
    /// Propagates any error returned by `set_touch_id_authentication_allowable_reuse_duration`.
    pub fn set_allowable_reuse_duration(&self, duration: f64) -> Result<()> {
        self.set_touch_id_authentication_allowable_reuse_duration(duration)
    }

    /// The framework-defined maximum reuse duration, in seconds.
    #[must_use]
    pub fn touch_id_authentication_maximum_allowable_reuse_duration() -> f64 {
        unsafe {
            ffi::la_context::la_context_get_touch_id_authentication_maximum_allowable_reuse_duration(
            )
        }
    }

    /// Read whether interactive authentication UI is disabled.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn interaction_not_allowed(&self) -> Result<bool> {
        bridge_bool(|out, error_out| unsafe {
            ffi::la_context::la_context_get_interaction_not_allowed(
                self.handle.as_ptr(),
                out,
                error_out,
            )
        })
    }

    /// Enable or disable interactive authentication UI.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn set_interaction_not_allowed(&self, value: bool) -> Result<()> {
        bridge_unit(|error_out| unsafe {
            ffi::la_context::la_context_set_interaction_not_allowed(
                self.handle.as_ptr(),
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
            ffi::la_context::la_context_get_biometry_type(self.handle.as_ptr(), out, error_out)
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
            ffi::la_context::la_context_get_evaluated_policy_domain_state(
                self.handle.as_ptr(),
                out,
                out_len,
                error_out,
            )
        })
    }

    /// Set an application-provided credential for subsequent authentication operations.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn set_credential(&self, credential: &LACredential) -> Result<bool> {
        let bytes = credential.bytes();
        bridge_bool(|out, error_out| unsafe {
            ffi::la_context::la_context_set_credential(
                self.handle.as_ptr(),
                bytes.as_ptr(),
                bytes.len(),
                credential.credential_type().as_ffi(),
                1,
                out,
                error_out,
            )
        })
    }

    /// Remove any previously-supplied credential of the given type.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn clear_credential(&self, credential_type: LACredentialType) -> Result<bool> {
        bridge_bool(|out, error_out| unsafe {
            ffi::la_context::la_context_set_credential(
                self.handle.as_ptr(),
                ptr::null(),
                0,
                credential_type.as_ffi(),
                0,
                out,
                error_out,
            )
        })
    }

    /// Check whether a credential of the given type is currently stored on this context.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn is_credential_set(&self, credential_type: LACredentialType) -> Result<bool> {
        bridge_bool(|out, error_out| unsafe {
            ffi::la_context::la_context_is_credential_set(
                self.handle.as_ptr(),
                credential_type.as_ffi(),
                out,
                error_out,
            )
        })
    }

    /// Read the richer `domainState` snapshot available on macOS 15 and newer.
    ///
    /// # Errors
    ///
    /// Returns an error if the property is unavailable or the Swift bridge rejects the request.
    pub fn domain_state(&self) -> Result<LADomainState> {
        let state_hash = bridge_opt_bytes(|out, out_len, error_out| unsafe {
            ffi::la_context::la_context_get_domain_state_hash(
                self.handle.as_ptr(),
                out,
                out_len,
                error_out,
            )
        })?;
        let biometry_type = BiometryType::from_ffi(bridge_i32(|out, error_out| unsafe {
            ffi::la_context::la_context_get_domain_state_biometry_type(
                self.handle.as_ptr(),
                out,
                error_out,
            )
        })?);
        let biometry_state_hash = bridge_opt_bytes(|out, out_len, error_out| unsafe {
            ffi::la_context::la_context_get_domain_state_biometry_hash(
                self.handle.as_ptr(),
                out,
                out_len,
                error_out,
            )
        })?;
        let companion_types_raw = bridge_i32_vec(|out, out_len, error_out| unsafe {
            ffi::la_context::la_context_get_domain_state_companion_types(
                self.handle.as_ptr(),
                out,
                out_len,
                error_out,
            )
        })?;
        let companion_types: Vec<LACompanionType> = companion_types_raw
            .into_iter()
            .map(LACompanionType::from_ffi)
            .collect();
        let companion_state_hash = bridge_opt_bytes(|out, out_len, error_out| unsafe {
            ffi::la_context::la_context_get_domain_state_companion_hash(
                self.handle.as_ptr(),
                out,
                out_len,
                error_out,
            )
        })?;

        let mut per_type_state_hashes = BTreeMap::new();
        for companion_type in &companion_types {
            if let Some(hash) = bridge_opt_bytes(|out, out_len, error_out| unsafe {
                ffi::la_context::la_context_get_domain_state_companion_hash_for_type(
                    self.handle.as_ptr(),
                    companion_type.raw_value(),
                    out,
                    out_len,
                    error_out,
                )
            })? {
                per_type_state_hashes.insert(*companion_type, hash);
            }
        }

        Ok(LADomainState {
            state_hash,
            biometry: LADomainStateBiometry {
                biometry_type,
                state_hash: biometry_state_hash,
            },
            companion: Some(LADomainStateCompanion {
                available_companion_types: companion_types,
                state_hash: companion_state_hash,
                per_type_state_hashes,
            }),
        })
    }

    /// Internal helper to get the raw pointer for FFI calls.
    ///
    /// Used by the async API module. This is intentionally non-public.
    #[cfg(feature = "async")]
    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}

#[cfg(test)]
mod tests {
    use super::{LACompanionType, LAContext, Result};
    use crate::{LACredential, LACredentialType, LAPolicy};

    #[test]
    fn property_round_trip_without_prompt() -> Result<()> {
        let context = LAContext::new()?;
        context.set_interaction_not_allowed(true)?;
        context.set_localized_fallback_title(Some("Use Password"))?;
        context.set_localized_cancel_title(Some("Cancel"))?;
        context.set_localized_reason("Test local authentication")?;
        context.set_allowable_reuse_duration(30.0)?;
        let credential = LACredential::application_password(b"secret".to_vec());

        assert!(context.set_credential(&credential)?);
        assert!(context.is_credential_set(LACredentialType::ApplicationPassword)?);
        assert!(context.clear_credential(LACredentialType::ApplicationPassword)?);
        assert!(!context.is_credential_set(LACredentialType::ApplicationPassword)?);
        assert!(context.interaction_not_allowed()?);
        assert_eq!(
            context.localized_fallback_title()?.as_deref(),
            Some("Use Password")
        );
        assert_eq!(context.localized_cancel_title()?.as_deref(), Some("Cancel"));
        assert_eq!(context.localized_reason()?, "Test local authentication");
        assert!((context.allowable_reuse_duration()? - 30.0).abs() < f64::EPSILON);
        assert!(LAContext::touch_id_authentication_maximum_allowable_reuse_duration() >= 300.0);

        let _ = context.can_evaluate_policy(LAPolicy::DeviceOwnerAuthenticationWithBiometrics);
        let domain_state = context.domain_state()?;
        let _ = domain_state.biometry().biometry_type();
        if let Some(companion) = domain_state.companion() {
            for companion_type in companion.available_companion_types() {
                let _ = companion.state_hash_for(*companion_type);
            }
            let _ = companion.state_hash_for(LACompanionType::Watch);
        }
        Ok(())
    }
}
