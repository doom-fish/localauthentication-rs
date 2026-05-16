use core::ffi::c_char;

pub const AUTHENTICATION_FAILED: i32 = -1;
pub const USER_CANCEL: i32 = -2;
pub const USER_FALLBACK: i32 = -3;
pub const SYSTEM_CANCEL: i32 = -4;
pub const PASSCODE_NOT_SET: i32 = -5;
pub const BIOMETRY_NOT_AVAILABLE: i32 = -6;
pub const BIOMETRY_NOT_ENROLLED: i32 = -7;
pub const BIOMETRY_LOCKOUT: i32 = -8;
pub const APP_CANCEL: i32 = -9;
pub const INVALID_CONTEXT: i32 = -10;
pub const WATCH_NOT_AVAILABLE: i32 = -11;
pub const COMPANION_NOT_AVAILABLE: i32 = -11;
pub const BIOMETRY_NOT_PAIRED: i32 = -12;
pub const BIOMETRY_DISCONNECTED: i32 = -13;
pub const INVALID_DIMENSIONS: i32 = -14;
pub const NOT_INTERACTIVE: i32 = -1004;

extern "C" {
    pub fn la_error_copy_domain(out_domain: *mut *mut c_char, error_out: *mut *mut c_char) -> i32;
}
