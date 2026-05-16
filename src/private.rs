use core::ffi::{c_char, c_void};
use std::ffi::CString;
use std::fmt;
use std::ptr;
use std::ptr::NonNull;

use libc::free;

use crate::ffi;
use crate::la_error::{
    from_status, from_status_message, take_owned_buffer, take_owned_c_string, LAError, Result,
};

pub struct OwnedHandle {
    raw: NonNull<c_void>,
    release: unsafe extern "C" fn(*mut c_void),
}

impl OwnedHandle {
    pub const fn new(raw: NonNull<c_void>, release: unsafe extern "C" fn(*mut c_void)) -> Self {
        Self { raw, release }
    }

    pub const fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

impl fmt::Debug for OwnedHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OwnedHandle")
            .field("raw", &self.raw)
            .finish_non_exhaustive()
    }
}

impl Drop for OwnedHandle {
    fn drop(&mut self) {
        unsafe { (self.release)(self.raw.as_ptr()) };
    }
}

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
        LAError::BridgeFailed("LocalAuthentication bridge returned a null object".to_owned())
    })
}

pub fn bridge_opt_ptr<F>(call: F) -> Result<Option<NonNull<c_void>>>
where
    F: FnOnce(*mut *mut c_void, *mut *mut c_char) -> i32,
{
    let mut out = ptr::null_mut();
    let mut error = ptr::null_mut();

    let status = call(&mut out, &mut error);
    if status != ffi::status::OK {
        return Err(from_status(status, error));
    }

    Ok(NonNull::new(out))
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

pub fn bridge_i64<F>(call: F) -> Result<i64>
where
    F: FnOnce(*mut i64, *mut *mut c_char) -> i32,
{
    let mut out = 0_i64;
    let mut error = ptr::null_mut();

    let status = call(&mut out, &mut error);
    if status != ffi::status::OK {
        return Err(from_status(status, error));
    }

    Ok(out)
}

pub fn bridge_string<F>(call: F) -> Result<String>
where
    F: FnOnce(*mut *mut c_char, *mut *mut c_char) -> i32,
{
    let mut out = ptr::null_mut();
    let mut error = ptr::null_mut();

    let status = call(&mut out, &mut error);
    if status != ffi::status::OK {
        return Err(from_status(status, error));
    }

    Ok(take_owned_c_string(out))
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

pub fn bridge_bytes<F>(call: F) -> Result<Vec<u8>>
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

    Ok(take_owned_buffer(out, out_len))
}

pub fn bridge_opt_bytes<F>(call: F) -> Result<Option<Vec<u8>>>
where
    F: FnOnce(*mut *mut u8, *mut usize, *mut *mut c_char) -> i32,
{
    let bytes = bridge_bytes(call)?;
    if bytes.is_empty() {
        Ok(None)
    } else {
        Ok(Some(bytes))
    }
}

pub fn bridge_i32_vec<F>(call: F) -> Result<Vec<i32>>
where
    F: FnOnce(*mut *mut i32, *mut usize, *mut *mut c_char) -> i32,
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
            unsafe { free(out.cast()) };
        }
        return Ok(Vec::new());
    }

    let values = unsafe { std::slice::from_raw_parts(out, out_len) }.to_vec();
    unsafe { free(out.cast()) };
    Ok(values)
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
        LAError::InvalidArgument(
            "strings passed to `LocalAuthentication` must not contain interior NUL bytes"
                .to_owned(),
        )
    })
}
