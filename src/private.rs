use core::ffi::{c_char, c_void};
use std::ffi::CString;
use std::ptr;
use std::ptr::NonNull;

use crate::error::{
    from_status, from_status_message, take_owned_buffer, take_owned_c_string,
    LocalAuthenticationError, Result,
};
use crate::ffi;

pub fn bridge_ptr<F>(call: F) -> Result<NonNull<c_void>>
where
    F: FnOnce(*mut *mut c_void, *mut *mut c_char) -> i32,
{
    let mut out = ptr::null_mut();
    let mut error = ptr::null_mut();

    let status = call(&mut out, &mut error);
    if status != ffi::status::OK {
        return Err(from_status(status, error));
    }

    NonNull::new(out).ok_or_else(|| {
        LocalAuthenticationError::BridgeFailed(
            "LocalAuthentication bridge returned a null object".to_owned(),
        )
    })
}

pub fn bridge_unit<F>(call: F) -> Result<()>
where
    F: FnOnce(*mut *mut c_char) -> i32,
{
    let mut error = ptr::null_mut();

    let status = call(&mut error);
    if status != ffi::status::OK {
        return Err(from_status(status, error));
    }

    Ok(())
}

pub fn bridge_bool<F>(call: F) -> Result<bool>
where
    F: FnOnce(*mut u8, *mut *mut c_char) -> i32,
{
    let mut out = 0_u8;
    let mut error = ptr::null_mut();

    let status = call(&mut out, &mut error);
    if status != ffi::status::OK {
        return Err(from_status(status, error));
    }

    Ok(out != 0)
}

pub fn bridge_f64<F>(call: F) -> Result<f64>
where
    F: FnOnce(*mut f64, *mut *mut c_char) -> i32,
{
    let mut out = 0.0_f64;
    let mut error = ptr::null_mut();

    let status = call(&mut out, &mut error);
    if status != ffi::status::OK {
        return Err(from_status(status, error));
    }

    Ok(out)
}

pub fn bridge_i32<F>(call: F) -> Result<i32>
where
    F: FnOnce(*mut i32, *mut *mut c_char) -> i32,
{
    let mut out = 0_i32;
    let mut error = ptr::null_mut();

    let status = call(&mut out, &mut error);
    if status != ffi::status::OK {
        return Err(from_status(status, error));
    }

    Ok(out)
}

pub fn bridge_opt_string<F>(call: F) -> Result<Option<String>>
where
    F: FnOnce(*mut *mut c_char, *mut *mut c_char) -> i32,
{
    let mut out = ptr::null_mut();
    let mut error = ptr::null_mut();

    let status = call(&mut out, &mut error);
    if status != ffi::status::OK {
        return Err(from_status(status, error));
    }

    if out.is_null() {
        Ok(None)
    } else {
        Ok(Some(take_owned_c_string(out)))
    }
}

pub fn bridge_opt_bytes<F>(call: F) -> Result<Option<Vec<u8>>>
where
    F: FnOnce(*mut *mut u8, *mut usize, *mut *mut c_char) -> i32,
{
    let mut out = ptr::null_mut();
    let mut out_len = 0_usize;
    let mut error = ptr::null_mut();

    let status = call(&mut out, &mut out_len, &mut error);
    if status != ffi::status::OK {
        return Err(from_status(status, error));
    }

    if out.is_null() || out_len == 0 {
        if !out.is_null() {
            let _ = take_owned_buffer(out, out_len);
        }
        Ok(None)
    } else {
        Ok(Some(take_owned_buffer(out, out_len)))
    }
}

pub fn framework_bool_result(
    value: bool,
    error_code: i32,
    error_message: *mut c_char,
) -> Result<bool> {
    if error_code == 0 {
        if !error_message.is_null() {
            let _ = take_owned_c_string(error_message);
        }
        Ok(value)
    } else {
        Err(from_status_message(
            error_code,
            take_owned_c_string(error_message),
        ))
    }
}

pub fn cstring(value: &str) -> Result<CString> {
    CString::new(value).map_err(|_| {
        LocalAuthenticationError::InvalidArgument(
            "strings passed to `LocalAuthentication` must not contain interior NUL bytes"
                .to_owned(),
        )
    })
}
