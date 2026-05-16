use core::ffi::{c_char, c_void};

extern "C" {
    pub fn la_persisted_right_release(right: *mut c_void);
    pub fn la_persisted_right_get_state(
        right: *mut c_void,
        out_state: *mut i32,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_persisted_right_get_tag(
        right: *mut c_void,
        out_tag: *mut i64,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_persisted_right_set_tag(
        right: *mut c_void,
        tag: i64,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_persisted_right_authorize(
        right: *mut c_void,
        localized_reason: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_persisted_right_check_can_authorize(
        right: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_persisted_right_deauthorize(right: *mut c_void, error_out: *mut *mut c_char) -> i32;
    pub fn la_persisted_right_get_key(
        right: *mut c_void,
        out_key: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_persisted_right_get_secret(
        right: *mut c_void,
        out_secret: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;

    pub fn la_secret_release(secret: *mut c_void);
    pub fn la_secret_load_data(
        secret: *mut c_void,
        out_bytes: *mut *mut u8,
        out_len: *mut usize,
        error_out: *mut *mut c_char,
    ) -> i32;

    pub fn la_private_key_release(key: *mut c_void);
    pub fn la_private_key_get_public_key(
        key: *mut c_void,
        out_public_key: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_private_key_can_sign_using_algorithm(
        key: *mut c_void,
        algorithm: *const c_char,
        out_value: *mut u8,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_private_key_sign_data(
        key: *mut c_void,
        data: *const u8,
        data_len: usize,
        algorithm: *const c_char,
        out_bytes: *mut *mut u8,
        out_len: *mut usize,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_private_key_can_decrypt_using_algorithm(
        key: *mut c_void,
        algorithm: *const c_char,
        out_value: *mut u8,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_private_key_decrypt_data(
        key: *mut c_void,
        data: *const u8,
        data_len: usize,
        algorithm: *const c_char,
        out_bytes: *mut *mut u8,
        out_len: *mut usize,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_private_key_can_exchange_keys_using_algorithm(
        key: *mut c_void,
        algorithm: *const c_char,
        out_value: *mut u8,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_private_key_exchange_keys_with_public_key(
        key: *mut c_void,
        public_key: *const u8,
        public_key_len: usize,
        algorithm: *const c_char,
        requested_size: i64,
        shared_info: *const u8,
        shared_info_len: usize,
        has_shared_info: u8,
        out_bytes: *mut *mut u8,
        out_len: *mut usize,
        error_out: *mut *mut c_char,
    ) -> i32;
}
