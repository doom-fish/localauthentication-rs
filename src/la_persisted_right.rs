//! `LAPersistedRight`, `LAPrivateKey`, and `LASecret` wrappers.

use crate::ffi;
use crate::la_error::{LAError, Result};
use crate::la_public_key::{LAPublicKey, SecKeyAlgorithm};
use crate::la_right::LARightState;
use crate::private::{
    bridge_bool, bridge_bytes, bridge_i32, bridge_i64, bridge_ptr, bridge_unit, cstring,
    OwnedHandle,
};

/// Managed wrapper around Apple's `LAPersistedRight`.
#[derive(Debug)]
pub struct LAPersistedRight {
    handle: OwnedHandle,
}

impl LAPersistedRight {
    pub(crate) fn from_raw(raw: std::ptr::NonNull<core::ffi::c_void>) -> Self {
        Self {
            handle: OwnedHandle::new(raw, ffi::la_persisted_right::la_persisted_right_release),
        }
    }

    pub(crate) const fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }

    /// The current authorization state.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn state(&self) -> Result<LARightState> {
        let raw = bridge_i32(|out, error_out| unsafe {
            ffi::la_persisted_right::la_persisted_right_get_state(
                self.handle.as_ptr(),
                out,
                error_out,
            )
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
            ffi::la_persisted_right::la_persisted_right_get_tag(
                self.handle.as_ptr(),
                out,
                error_out,
            )
        })
    }

    /// Update the application-controlled integer tag.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn set_tag(&self, tag: i64) -> Result<()> {
        bridge_unit(|error_out| unsafe {
            ffi::la_persisted_right::la_persisted_right_set_tag(
                self.handle.as_ptr(),
                tag,
                error_out,
            )
        })
    }

    /// Attempt to authorize the persisted right.
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
            ffi::la_persisted_right::la_persisted_right_authorize(
                self.handle.as_ptr(),
                localized_reason.as_ptr(),
                error_out,
            )
        })
    }

    /// Preflight whether the persisted right can eventually be authorized.
    ///
    /// # Errors
    ///
    /// Returns a mapped framework or bridge error when authorization is not possible.
    pub fn check_can_authorize(&self) -> Result<()> {
        bridge_unit(|error_out| unsafe {
            ffi::la_persisted_right::la_persisted_right_check_can_authorize(
                self.handle.as_ptr(),
                error_out,
            )
        })
    }

    /// Deauthorize the persisted right.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn deauthorize(&self) -> Result<()> {
        bridge_unit(|error_out| unsafe {
            ffi::la_persisted_right::la_persisted_right_deauthorize(self.handle.as_ptr(), error_out)
        })
    }

    /// Borrow the managed private key associated with this persisted right.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn key(&self) -> Result<LAPrivateKey> {
        Ok(LAPrivateKey::from_raw(bridge_ptr(
            |out, error_out| unsafe {
                ffi::la_persisted_right::la_persisted_right_get_key(
                    self.handle.as_ptr(),
                    out,
                    error_out,
                )
            },
        )?))
    }

    /// Borrow the generic secret associated with this persisted right.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn secret(&self) -> Result<LASecret> {
        Ok(LASecret::from_raw(bridge_ptr(|out, error_out| unsafe {
            ffi::la_persisted_right::la_persisted_right_get_secret(
                self.handle.as_ptr(),
                out,
                error_out,
            )
        })?))
    }

    /// Convenience helper returning `self.key()?.public_key()`.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn public_key(&self) -> Result<LAPublicKey> {
        self.key()?.public_key()
    }
}

/// Managed wrapper around Apple's `LASecret`.
#[derive(Debug)]
pub struct LASecret {
    handle: OwnedHandle,
}

impl LASecret {
    pub(crate) fn from_raw(raw: std::ptr::NonNull<core::ffi::c_void>) -> Self {
        Self {
            handle: OwnedHandle::new(raw, ffi::la_persisted_right::la_secret_release),
        }
    }

    /// Load the secret bytes.
    ///
    /// # Errors
    ///
    /// Returns a mapped framework or bridge error if loading fails.
    pub fn load_data(&self) -> Result<Vec<u8>> {
        bridge_bytes(|out, out_len, error_out| unsafe {
            ffi::la_persisted_right::la_secret_load_data(
                self.handle.as_ptr(),
                out,
                out_len,
                error_out,
            )
        })
    }
}

/// Managed wrapper around Apple's `LAPrivateKey`.
#[derive(Debug)]
pub struct LAPrivateKey {
    handle: OwnedHandle,
}

impl LAPrivateKey {
    pub(crate) fn from_raw(raw: std::ptr::NonNull<core::ffi::c_void>) -> Self {
        Self {
            handle: OwnedHandle::new(raw, ffi::la_persisted_right::la_private_key_release),
        }
    }

    /// Borrow the public-key counterpart of this private key.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn public_key(&self) -> Result<LAPublicKey> {
        Ok(LAPublicKey::from_raw(bridge_ptr(
            |out, error_out| unsafe {
                ffi::la_persisted_right::la_private_key_get_public_key(
                    self.handle.as_ptr(),
                    out,
                    error_out,
                )
            },
        )?))
    }

    /// Check whether an algorithm can sign with this key.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn can_sign_using(&self, algorithm: &SecKeyAlgorithm) -> Result<bool> {
        let algorithm = cstring(algorithm.raw_name())?;
        bridge_bool(|out, error_out| unsafe {
            ffi::la_persisted_right::la_private_key_can_sign_using_algorithm(
                self.handle.as_ptr(),
                algorithm.as_ptr(),
                out,
                error_out,
            )
        })
    }

    /// Sign data with this key.
    ///
    /// # Errors
    ///
    /// Returns a mapped framework or bridge error if signing fails.
    pub fn sign(&self, data: &[u8], algorithm: &SecKeyAlgorithm) -> Result<Vec<u8>> {
        let algorithm = cstring(algorithm.raw_name())?;
        bridge_bytes(|out, out_len, error_out| unsafe {
            ffi::la_persisted_right::la_private_key_sign_data(
                self.handle.as_ptr(),
                data.as_ptr(),
                data.len(),
                algorithm.as_ptr(),
                out,
                out_len,
                error_out,
            )
        })
    }

    /// Check whether an algorithm can decrypt with this key.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn can_decrypt_using(&self, algorithm: &SecKeyAlgorithm) -> Result<bool> {
        let algorithm = cstring(algorithm.raw_name())?;
        bridge_bool(|out, error_out| unsafe {
            ffi::la_persisted_right::la_private_key_can_decrypt_using_algorithm(
                self.handle.as_ptr(),
                algorithm.as_ptr(),
                out,
                error_out,
            )
        })
    }

    /// Decrypt data with this key.
    ///
    /// # Errors
    ///
    /// Returns a mapped framework or bridge error if decryption fails.
    pub fn decrypt(&self, data: &[u8], algorithm: &SecKeyAlgorithm) -> Result<Vec<u8>> {
        let algorithm = cstring(algorithm.raw_name())?;
        bridge_bytes(|out, out_len, error_out| unsafe {
            ffi::la_persisted_right::la_private_key_decrypt_data(
                self.handle.as_ptr(),
                data.as_ptr(),
                data.len(),
                algorithm.as_ptr(),
                out,
                out_len,
                error_out,
            )
        })
    }

    /// Check whether an algorithm can be used for key exchange.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn can_exchange_keys_using(&self, algorithm: &SecKeyAlgorithm) -> Result<bool> {
        let algorithm = cstring(algorithm.raw_name())?;
        bridge_bool(|out, error_out| unsafe {
            ffi::la_persisted_right::la_private_key_can_exchange_keys_using_algorithm(
                self.handle.as_ptr(),
                algorithm.as_ptr(),
                out,
                error_out,
            )
        })
    }
}
