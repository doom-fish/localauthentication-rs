#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API documentation
//!
//! Safe Rust bindings for Apple's `LocalAuthentication.framework` on macOS.

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod context;
pub mod error;
pub mod ffi;
pub mod la_authentication_requirement;
pub mod la_context;
pub mod la_credential;
pub mod la_environment;
pub mod la_error;
pub mod la_persisted_right;
pub mod la_policy;
pub mod la_public_key;
pub mod la_right;
pub mod la_right_store;
mod private;

#[cfg(feature = "async")]
pub mod async_api;

pub use la_authentication_requirement::{
    LAAuthenticationRequirement, LABiometryFallbackRequirement,
};
pub use la_context::{
    BiometryType, LAAccessControlOperation, LACompanionType, LAContext, LADomainState,
    LADomainStateBiometry, LADomainStateCompanion,
};
pub use la_credential::{LACredential, LACredentialType};
pub use la_environment::{
    LAEnvironment, LAEnvironmentMechanism, LAEnvironmentMechanismBiometry,
    LAEnvironmentMechanismCompanion, LAEnvironmentMechanismUserPassword,
    LAEnvironmentObserver, LAEnvironmentObserverRegistration, LAEnvironmentState,
};
pub use la_error::{LAError, LocalAuthenticationError, Result, LA_ERROR_DOMAIN};
pub use la_persisted_right::{LAPersistedRight, LAPrivateKey, LASecret};
pub use la_policy::{LAPolicy, Policy};
pub use la_public_key::{LAPublicKey, SecKeyAlgorithm, SecKeyExchangeParameters};
pub use la_right::{LARight, LARightState};
pub use la_right_store::LARightStore;

/// Common imports for users of this crate.
pub mod prelude {
    pub use crate::la_authentication_requirement::{
        LAAuthenticationRequirement, LABiometryFallbackRequirement,
    };
    pub use crate::la_context::{
        BiometryType, LAAccessControlOperation, LACompanionType, LAContext, LADomainState,
        LADomainStateBiometry, LADomainStateCompanion,
    };
    pub use crate::la_credential::{LACredential, LACredentialType};
    pub use crate::la_environment::{
        LAEnvironment, LAEnvironmentMechanism, LAEnvironmentMechanismBiometry,
        LAEnvironmentMechanismCompanion, LAEnvironmentMechanismUserPassword,
        LAEnvironmentObserver, LAEnvironmentObserverRegistration, LAEnvironmentState,
    };
    pub use crate::la_error::{LAError, LocalAuthenticationError, Result, LA_ERROR_DOMAIN};
    pub use crate::la_persisted_right::{LAPersistedRight, LAPrivateKey, LASecret};
    pub use crate::la_policy::{LAPolicy, Policy};
    pub use crate::la_public_key::{LAPublicKey, SecKeyAlgorithm, SecKeyExchangeParameters};
    pub use crate::la_right::{LARight, LARightState};
    pub use crate::la_right_store::LARightStore;
}
