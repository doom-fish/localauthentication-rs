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
mod private;

pub use context::{BiometryType, LAContext, Policy};
pub use error::{LocalAuthenticationError, Result};

/// Common imports for users of this crate.
pub mod prelude {
    pub use crate::context::{BiometryType, LAContext, Policy};
    pub use crate::error::{LocalAuthenticationError, Result};
}
