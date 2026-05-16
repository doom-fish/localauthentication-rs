use core::ffi::{c_char, c_void};

extern "C" {
    pub fn la_right_store_shared(out_store: *mut *mut c_void, error_out: *mut *mut c_char) -> i32;
    pub fn la_right_store_release(store: *mut c_void);
    pub fn la_right_store_right_for_identifier(
        store: *mut c_void,
        identifier: *const c_char,
        out_right: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_right_store_save_right(
        store: *mut c_void,
        right: *mut c_void,
        identifier: *const c_char,
        out_right: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_right_store_save_right_with_secret(
        store: *mut c_void,
        right: *mut c_void,
        identifier: *const c_char,
        secret_bytes: *const u8,
        secret_len: usize,
        out_right: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_right_store_remove_right(
        store: *mut c_void,
        right: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_right_store_remove_right_for_identifier(
        store: *mut c_void,
        identifier: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_right_store_remove_all_rights(store: *mut c_void, error_out: *mut *mut c_char)
        -> i32;
}
