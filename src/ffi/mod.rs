//! Raw FFI declarations matching the Swift `la_*` bridge exports.

#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    missing_docs
)]

pub mod la_authentication_requirement;
pub mod la_context;
pub mod la_credential;
pub mod la_error;
pub mod la_persisted_right;
pub mod la_policy;
pub mod la_public_key;
pub mod la_right;
pub mod la_right_store;

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -10_000;
    pub const TIMED_OUT: i32 = -10_001;
    pub const BRIDGE_FAILED: i32 = -10_002;
    pub const UNKNOWN: i32 = -10_099;
}

pub mod biometry {
    pub const NONE: i32 = 0;
    pub const TOUCH_ID: i32 = 1;
    pub const FACE_ID: i32 = 2;
    pub const OPTIC_ID: i32 = 4;
}

pub mod companion {
    pub const WATCH: i32 = 1;
    pub const MAC: i32 = 2;
    pub const VISION: i32 = 4;
}
