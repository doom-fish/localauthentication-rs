//! Application-provided credential helpers for `LAContext`.

use std::fmt;

use zeroize::Zeroizing;

use crate::ffi;

/// Credential kinds accepted by `LAContext::set_credential`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LACredentialType {
    /// Application-provided password data.
    ApplicationPassword,
    /// Application-provided smart-card PIN data.
    SmartCardPin,
}

impl LACredentialType {
    #[must_use]
    pub const fn raw_value(self) -> i32 {
        match self {
            Self::ApplicationPassword => ffi::la_credential::APPLICATION_PASSWORD,
            Self::SmartCardPin => ffi::la_credential::SMART_CARD_PIN,
        }
    }

    pub(crate) const fn as_ffi(self) -> i32 {
        self.raw_value()
    }
}

/// Owned credential bytes paired with their `LACredentialType`.
#[derive(Clone)]
pub struct LACredential {
    credential_type: LACredentialType,
    bytes: Zeroizing<Vec<u8>>,
}

impl LACredential {
    /// Create a credential from raw bytes.
    #[must_use]
    pub fn new(credential_type: LACredentialType, bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            credential_type,
            bytes: Zeroizing::new(bytes.into()),
        }
    }

    /// Create an application-password credential.
    #[must_use]
    pub fn application_password(bytes: impl Into<Vec<u8>>) -> Self {
        Self::new(LACredentialType::ApplicationPassword, bytes)
    }

    /// Create a smart-card-PIN credential.
    #[must_use]
    pub fn smart_card_pin(bytes: impl Into<Vec<u8>>) -> Self {
        Self::new(LACredentialType::SmartCardPin, bytes)
    }

    /// Return the credential kind.
    #[must_use]
    pub const fn credential_type(&self) -> LACredentialType {
        self.credential_type
    }

    /// Borrow the credential bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl fmt::Debug for LACredential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LACredential")
            .field("credential_type", &self.credential_type)
            .field("bytes", &"<redacted>")
            .finish()
    }
}

impl PartialEq for LACredential {
    fn eq(&self, other: &Self) -> bool {
        let same_bytes = constant_time_eq(&self.bytes, &other.bytes);
        same_bytes && self.credential_type == other.credential_type
    }
}

impl Eq for LACredential {}

fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    let difference = left
        .iter()
        .zip(right)
        .fold(0_u8, |difference, (left, right)| {
            difference | (left ^ right)
        });
    std::hint::black_box(difference) == 0
}
