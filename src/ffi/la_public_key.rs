use core::ffi::{c_char, c_void};

extern "C" {
    pub fn la_public_key_release(key: *mut c_void);
    pub fn la_public_key_export_bytes(
        key: *mut c_void,
        out_bytes: *mut *mut u8,
        out_len: *mut usize,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_public_key_can_encrypt_using_algorithm(
        key: *mut c_void,
        algorithm: *const c_char,
        out_value: *mut u8,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_public_key_encrypt_data(
        key: *mut c_void,
        data: *const u8,
        data_len: usize,
        algorithm: *const c_char,
        out_bytes: *mut *mut u8,
        out_len: *mut usize,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_public_key_can_verify_using_algorithm(
        key: *mut c_void,
        algorithm: *const c_char,
        out_value: *mut u8,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_public_key_verify_data(
        key: *mut c_void,
        signed_data: *const u8,
        signed_data_len: usize,
        signature: *const u8,
        signature_len: usize,
        algorithm: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
}
