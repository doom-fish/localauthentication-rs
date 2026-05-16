//! `LAPublicKey` wrappers.

use crate::ffi;
use crate::la_error::Result;
use crate::private::{bridge_bool, bridge_bytes, bridge_unit, cstring, OwnedHandle};

/// String-backed `SecKeyAlgorithm` wrapper used by the `LocalAuthentication` key APIs.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SecKeyAlgorithm(String);

impl SecKeyAlgorithm {
    /// Create an algorithm from the raw `SecKeyAlgorithm` name.
    #[must_use]
    pub fn from_raw_name(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    /// Borrow the raw `SecKeyAlgorithm` name.
    #[must_use]
    pub fn raw_name(&self) -> &str {
        &self.0
    }

    /// `SecKeyAlgorithm.ecdsaSignatureMessageX962SHA256`.
    #[must_use]
    pub fn ecdsa_signature_message_x962_sha256() -> Self {
        Self::from_raw_name("algid:sign:ECDSA:message-X962:SHA256")
    }

    /// `SecKeyAlgorithm.ecdsaSignatureDigestX962SHA256`.
    #[must_use]
    pub fn ecdsa_signature_digest_x962_sha256() -> Self {
        Self::from_raw_name("algid:sign:ECDSA:digest-X962:SHA256")
    }

    /// `SecKeyAlgorithm.eciesEncryptionStandardVariableIVX963SHA256AESGCM`.
    #[must_use]
    pub fn ecies_encryption_standard_variable_iv_x963_sha256_aes_gcm() -> Self {
        Self::from_raw_name("algid:encrypt:ECIES:ECDH:KDFX963:SHA256:AESGCM-KDFIV")
    }

    /// `SecKeyAlgorithm.eciesEncryptionCofactorVariableIVX963SHA256AESGCM`.
    #[must_use]
    pub fn ecies_encryption_cofactor_variable_iv_x963_sha256_aes_gcm() -> Self {
        Self::from_raw_name("algid:encrypt:ECIES:ECDHC:KDFX963:SHA256:AESGCM-KDFIV")
    }

    /// `SecKeyAlgorithm.ecdhKeyExchangeCofactorX963SHA256`.
    #[must_use]
    pub fn ecdh_key_exchange_cofactor_x963_sha256() -> Self {
        Self::from_raw_name("algid:keyexchange:ECDHC:KDFX963:SHA256")
    }
}

impl From<&str> for SecKeyAlgorithm {
    fn from(value: &str) -> Self {
        Self::from_raw_name(value)
    }
}

impl From<String> for SecKeyAlgorithm {
    fn from(value: String) -> Self {
        Self::from_raw_name(value)
    }
}

/// Managed wrapper around Apple's `LAPublicKey`.
#[derive(Debug)]
pub struct LAPublicKey {
    handle: OwnedHandle,
}

impl LAPublicKey {
    pub(crate) fn from_raw(raw: std::ptr::NonNull<core::ffi::c_void>) -> Self {
        Self {
            handle: OwnedHandle::new(raw, ffi::la_public_key::la_public_key_release),
        }
    }

    /// Export the public-key bytes.
    ///
    /// # Errors
    ///
    /// Returns a mapped framework or bridge error if export fails.
    pub fn export_bytes(&self) -> Result<Vec<u8>> {
        bridge_bytes(|out, out_len, error_out| unsafe {
            ffi::la_public_key::la_public_key_export_bytes(
                self.handle.as_ptr(),
                out,
                out_len,
                error_out,
            )
        })
    }

    /// Check whether an algorithm can encrypt with this key.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn can_encrypt_using(&self, algorithm: &SecKeyAlgorithm) -> Result<bool> {
        let algorithm = cstring(algorithm.raw_name())?;
        bridge_bool(|out, error_out| unsafe {
            ffi::la_public_key::la_public_key_can_encrypt_using_algorithm(
                self.handle.as_ptr(),
                algorithm.as_ptr(),
                out,
                error_out,
            )
        })
    }

    /// Encrypt data with this key.
    ///
    /// # Errors
    ///
    /// Returns a mapped framework or bridge error if encryption fails.
    pub fn encrypt(&self, data: &[u8], algorithm: &SecKeyAlgorithm) -> Result<Vec<u8>> {
        let algorithm = cstring(algorithm.raw_name())?;
        bridge_bytes(|out, out_len, error_out| unsafe {
            ffi::la_public_key::la_public_key_encrypt_data(
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

    /// Check whether an algorithm can verify signatures with this key.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn can_verify_using(&self, algorithm: &SecKeyAlgorithm) -> Result<bool> {
        let algorithm = cstring(algorithm.raw_name())?;
        bridge_bool(|out, error_out| unsafe {
            ffi::la_public_key::la_public_key_can_verify_using_algorithm(
                self.handle.as_ptr(),
                algorithm.as_ptr(),
                out,
                error_out,
            )
        })
    }

    /// Verify a signature with this key.
    ///
    /// # Errors
    ///
    /// Returns a mapped framework or bridge error if verification fails.
    pub fn verify(
        &self,
        signed_data: &[u8],
        signature: &[u8],
        algorithm: &SecKeyAlgorithm,
    ) -> Result<()> {
        let algorithm = cstring(algorithm.raw_name())?;
        bridge_unit(|error_out| unsafe {
            ffi::la_public_key::la_public_key_verify_data(
                self.handle.as_ptr(),
                signed_data.as_ptr(),
                signed_data.len(),
                signature.as_ptr(),
                signature.len(),
                algorithm.as_ptr(),
                error_out,
            )
        })
    }
}
