//! `LARightStore` wrappers.

use crate::ffi;
use crate::la_error::Result;
use crate::la_persisted_right::LAPersistedRight;
use crate::la_right::LARight;
use crate::private::{bridge_ptr, bridge_unit, cstring, OwnedHandle};

/// Managed wrapper around Apple's singleton `LARightStore`.
#[derive(Debug)]
pub struct LARightStore {
    handle: OwnedHandle,
}

impl LARightStore {
    /// Retain the framework singleton store.
    ///
    /// # Errors
    ///
    /// Returns an error if the API is unavailable or the Swift bridge rejects the request.
    pub fn shared() -> Result<Self> {
        let raw = bridge_ptr(|out, error_out| unsafe {
            ffi::la_right_store::la_right_store_shared(out, error_out)
        })?;
        Ok(Self {
            handle: OwnedHandle::new(raw, ffi::la_right_store::la_right_store_release),
        })
    }

    /// Fetch a persisted right by identifier.
    ///
    /// # Errors
    ///
    /// Returns a mapped framework or bridge error if the identifier does not resolve.
    pub fn right_for_identifier(&self, identifier: &str) -> Result<LAPersistedRight> {
        let identifier = cstring(identifier)?;
        Ok(LAPersistedRight::from_raw(bridge_ptr(
            |out, error_out| unsafe {
                ffi::la_right_store::la_right_store_right_for_identifier(
                    self.handle.as_ptr(),
                    identifier.as_ptr(),
                    out,
                    error_out,
                )
            },
        )?))
    }

    /// Persist a right for later reuse.
    ///
    /// # Errors
    ///
    /// Returns a mapped framework or bridge error if persistence fails.
    pub fn save_right(&self, right: &LARight, identifier: &str) -> Result<LAPersistedRight> {
        let identifier = cstring(identifier)?;
        Ok(LAPersistedRight::from_raw(bridge_ptr(
            |out, error_out| unsafe {
                ffi::la_right_store::la_right_store_save_right(
                    self.handle.as_ptr(),
                    right.as_ptr(),
                    identifier.as_ptr(),
                    out,
                    error_out,
                )
            },
        )?))
    }

    /// Persist a right together with secret data.
    ///
    /// # Errors
    ///
    /// Returns a mapped framework or bridge error if persistence fails.
    pub fn save_right_with_secret(
        &self,
        right: &LARight,
        identifier: &str,
        secret: &[u8],
    ) -> Result<LAPersistedRight> {
        let identifier = cstring(identifier)?;
        Ok(LAPersistedRight::from_raw(bridge_ptr(
            |out, error_out| unsafe {
                ffi::la_right_store::la_right_store_save_right_with_secret(
                    self.handle.as_ptr(),
                    right.as_ptr(),
                    identifier.as_ptr(),
                    secret.as_ptr(),
                    secret.len(),
                    out,
                    error_out,
                )
            },
        )?))
    }

    /// Remove a persisted right.
    ///
    /// # Errors
    ///
    /// Returns a mapped framework or bridge error if removal fails.
    pub fn remove_right(&self, right: &LAPersistedRight) -> Result<()> {
        bridge_unit(|error_out| unsafe {
            ffi::la_right_store::la_right_store_remove_right(
                self.handle.as_ptr(),
                right.as_ptr(),
                error_out,
            )
        })
    }

    /// Remove a persisted right by identifier.
    ///
    /// # Errors
    ///
    /// Returns a mapped framework or bridge error if removal fails.
    pub fn remove_right_for_identifier(&self, identifier: &str) -> Result<()> {
        let identifier = cstring(identifier)?;
        bridge_unit(|error_out| unsafe {
            ffi::la_right_store::la_right_store_remove_right_for_identifier(
                self.handle.as_ptr(),
                identifier.as_ptr(),
                error_out,
            )
        })
    }

    /// Remove all rights owned by the current client.
    ///
    /// # Errors
    ///
    /// Returns a mapped framework or bridge error if removal fails.
    pub fn remove_all_rights(&self) -> Result<()> {
        bridge_unit(|error_out| unsafe {
            ffi::la_right_store::la_right_store_remove_all_rights(self.handle.as_ptr(), error_out)
        })
    }
}
