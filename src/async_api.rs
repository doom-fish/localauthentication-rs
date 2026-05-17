//! Async API for `LocalAuthentication`
//!
//! This module provides async versions of authentication operations when the `async` feature is enabled.
//! The async API is **executor-agnostic** and works with any async runtime (Tokio, async-std, smol, etc.).

use crate::la_context::LAContext;
use crate::la_error::Result;
use crate::la_policy::LAPolicy;
use doom_fish_utils::completion::{error_from_cstr, AsyncCompletion, AsyncCompletionFuture};
use std::ffi::c_void;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// ============================================================================
// Callbacks for async operations
// ============================================================================

extern "C" fn evaluate_policy_callback(
    success: u8,
    error: *const i8,
    user_data: *mut c_void,
) {
    if error.is_null() {
        // SAFETY: user_data points to a valid AsyncCompletion<bool> created by AsyncCompletion::create()
        unsafe { AsyncCompletion::complete_ok(user_data, success != 0) };
    } else {
        // SAFETY: error is a valid C string from Swift bridge
        let error_msg = unsafe { error_from_cstr(error) };
        // SAFETY: user_data points to a valid AsyncCompletion<bool> created by AsyncCompletion::create()
        unsafe { AsyncCompletion::<bool>::complete_err(user_data, error_msg) };
    }
}

extern "C" fn evaluate_access_control_callback(
    success: u8,
    error: *const i8,
    user_data: *mut c_void,
) {
    if error.is_null() {
        // SAFETY: user_data points to a valid AsyncCompletion<bool> created by AsyncCompletion::create()
        unsafe { AsyncCompletion::complete_ok(user_data, success != 0) };
    } else {
        // SAFETY: error is a valid C string from Swift bridge
        let error_msg = unsafe { error_from_cstr(error) };
        // SAFETY: user_data points to a valid AsyncCompletion<bool> created by AsyncCompletion::create()
        unsafe { AsyncCompletion::<bool>::complete_err(user_data, error_msg) };
    }
}

// ============================================================================
// Future types
// ============================================================================

/// Future for async policy evaluation
pub struct AsyncPolicyEvaluation {
    inner: AsyncCompletionFuture<bool>,
}

impl std::fmt::Debug for AsyncPolicyEvaluation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncPolicyEvaluation")
            .finish_non_exhaustive()
    }
}

impl Future for AsyncPolicyEvaluation {
    type Output = Result<bool>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(crate::la_error::LAError::BridgeFailed))
    }
}

/// Future for async access control evaluation
pub struct AsyncAccessControlEvaluation {
    inner: AsyncCompletionFuture<bool>,
}

impl std::fmt::Debug for AsyncAccessControlEvaluation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncAccessControlEvaluation")
            .finish_non_exhaustive()
    }
}

impl Future for AsyncAccessControlEvaluation {
    type Output = Result<bool>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(crate::la_error::LAError::BridgeFailed))
    }
}

// ============================================================================
// Async operations extension trait
// ============================================================================

/// Extension trait adding async methods to `LAContext`
pub trait AsyncContextExt {
    /// Asynchronously evaluate a policy
    ///
    /// # Errors
    ///
    /// Returns an error if the localized reason is empty or contains a null byte.
    fn evaluate_policy_async(
        &self,
        policy: LAPolicy,
        localized_reason: &str,
    ) -> Result<AsyncPolicyEvaluation>;

    /// Asynchronously evaluate an access control
    ///
    /// # Safety
    ///
    /// The `access_control` pointer must be a valid, properly initialized `SecAccessControl` reference.
    ///
    /// # Errors
    ///
    /// Returns an error if the access control is null, localized reason is empty, or contains a null byte.
    unsafe fn evaluate_access_control_async(
        &self,
        access_control: *const c_void,
        operation: crate::la_context::LAAccessControlOperation,
        localized_reason: &str,
    ) -> Result<AsyncAccessControlEvaluation>;
}

impl AsyncContextExt for LAContext {
    /// Asynchronously evaluate a policy
    ///
    /// Uses callback-based Swift FFI for true async operation.
    ///
    /// # Arguments
    ///
    /// * `policy` - The authentication policy to evaluate
    /// * `localized_reason` - A localized reason shown to the user
    fn evaluate_policy_async(
        &self,
        policy: LAPolicy,
        localized_reason: &str,
    ) -> Result<AsyncPolicyEvaluation> {
        if localized_reason.is_empty() {
            return Err(crate::la_error::LAError::InvalidArgument(
                "localized reason must not be empty".to_owned(),
            ));
        }

        let (future, ctx) = AsyncCompletion::create();
        let reason_cstring = std::ffi::CString::new(localized_reason).map_err(|_| {
            crate::la_error::LAError::InvalidArgument("localized reason contains null byte".to_owned())
        })?;

        let context_ptr = self.as_ptr();

        unsafe {
            crate::ffi::la_context::la_context_evaluate_policy_async(
                context_ptr,
                policy.as_ffi(),
                reason_cstring.as_ptr(),
                evaluate_policy_callback,
                ctx,
            );
        }

        Ok(AsyncPolicyEvaluation { inner: future })
    }

    /// Asynchronously evaluate an access control
    ///
    /// Uses callback-based Swift FFI for true async operation.
    ///
    /// # Arguments
    ///
    /// * `access_control` - A `SecAccessControl` reference (as raw pointer)
    /// * `operation` - The access control operation to evaluate
    /// * `localized_reason` - A localized reason shown to the user
    ///
    /// # Safety
    ///
    /// The `access_control` pointer must be a valid, properly initialized `SecAccessControl` reference.
    unsafe fn evaluate_access_control_async(
        &self,
        access_control: *const c_void,
        operation: crate::la_context::LAAccessControlOperation,
        localized_reason: &str,
    ) -> Result<AsyncAccessControlEvaluation> {
        if access_control.is_null() {
            return Err(crate::la_error::LAError::InvalidArgument(
                "access control pointer must not be null".to_owned(),
            ));
        }
        if localized_reason.is_empty() {
            return Err(crate::la_error::LAError::InvalidArgument(
                "localized reason must not be empty".to_owned(),
            ));
        }

        let (future, ctx) = AsyncCompletion::create();
        let reason_cstring = std::ffi::CString::new(localized_reason).map_err(|_| {
            crate::la_error::LAError::InvalidArgument("localized reason contains null byte".to_owned())
        })?;

        let context_ptr = self.as_ptr();

        unsafe {
            crate::ffi::la_context::la_context_evaluate_access_control_async(
                context_ptr,
                access_control,
                operation.raw_value(),
                reason_cstring.as_ptr(),
                evaluate_access_control_callback,
                ctx,
            );
        }

        Ok(AsyncAccessControlEvaluation { inner: future })
    }
}
