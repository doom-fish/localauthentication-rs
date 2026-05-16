//! `LAEnvironment` observer, state, and mechanism wrappers.

use core::ffi::c_void;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr::NonNull;

use crate::ffi;
use crate::la_context::{BiometryType, LACompanionType};
use crate::la_error::{LAError, Result};
use crate::private::{
    bridge_bool, bridge_bytes, bridge_i32, bridge_i64, bridge_opt_bytes, bridge_opt_ptr,
    bridge_ptr, bridge_string, bridge_unit, OwnedHandle,
};

fn mechanism_is_usable(ptr: *mut c_void) -> Result<bool> {
    bridge_bool(|out, error_out| unsafe {
        ffi::la_environment::la_environment_mechanism_get_is_usable(ptr, out, error_out)
    })
}

fn mechanism_localized_name(ptr: *mut c_void) -> Result<String> {
    bridge_string(|out, error_out| unsafe {
        ffi::la_environment::la_environment_mechanism_get_localized_name(ptr, out, error_out)
    })
}

fn mechanism_icon_system_name(ptr: *mut c_void) -> Result<String> {
    bridge_string(|out, error_out| unsafe {
        ffi::la_environment::la_environment_mechanism_get_icon_system_name(ptr, out, error_out)
    })
}

fn count_to_usize(count: i64, label: &str) -> Result<usize> {
    usize::try_from(count)
        .map_err(|_| LAError::BridgeFailed(format!("LocalAuthentication returned an invalid {label} count")))
}

/// Observer callbacks for `LAEnvironment` state changes.
pub trait LAEnvironmentObserver: Send + Sync + 'static {
    /// Invoked after `environment` has transitioned away from `old_state`.
    fn state_did_change(&self, environment: &LAEnvironment, old_state: &LAEnvironmentState);
}

impl<F> LAEnvironmentObserver for F
where
    F: Fn(&LAEnvironment, &LAEnvironmentState) + Send + Sync + 'static,
{
    fn state_did_change(&self, environment: &LAEnvironment, old_state: &LAEnvironmentState) {
        self(environment, old_state);
    }
}

struct EnvironmentObserverContext {
    observer: Box<dyn LAEnvironmentObserver>,
}

unsafe extern "C" fn environment_observer_trampoline(
    context: *mut c_void,
    environment_ptr: *mut c_void,
    old_state_ptr: *mut c_void,
) {
    let Some(context) = NonNull::new(context.cast::<EnvironmentObserverContext>()) else {
        return;
    };
    let Some(environment_raw) = NonNull::new(environment_ptr) else {
        return;
    };
    let Some(old_state_raw) = NonNull::new(old_state_ptr) else {
        return;
    };

    let context = unsafe { context.as_ref() };
    let environment = LAEnvironment::from_raw(environment_raw);
    let old_state = LAEnvironmentState::from_raw(old_state_raw);

    let _ = catch_unwind(AssertUnwindSafe(|| {
        context.observer.state_did_change(&environment, &old_state);
    }));
}

unsafe extern "C" fn environment_observer_release(context: *mut c_void) {
    if let Some(context) = NonNull::new(context.cast::<EnvironmentObserverContext>()) {
        unsafe { drop(Box::from_raw(context.as_ptr())) };
    }
}

/// Strong registration that keeps an `LAEnvironmentObserver` alive while `LAEnvironment` only holds it weakly.
#[derive(Debug)]
pub struct LAEnvironmentObserverRegistration {
    handle: OwnedHandle,
}

impl LAEnvironmentObserverRegistration {
    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}

/// Managed wrapper around Apple's `LAEnvironment`.
#[derive(Debug)]
pub struct LAEnvironment {
    handle: OwnedHandle,
}

impl LAEnvironment {
    pub(crate) fn from_raw(raw: NonNull<c_void>) -> Self {
        Self {
            handle: OwnedHandle::new(raw, ffi::la_environment::la_environment_release),
        }
    }

    #[allow(dead_code)]
    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    /// The current user's authentication environment.
    ///
    /// # Errors
    ///
    /// Returns an error if the macOS 15 environment APIs are unavailable or the Swift bridge rejects the request.
    pub fn current_user() -> Result<Self> {
        Ok(Self::from_raw(bridge_ptr(|out, error_out| unsafe {
            ffi::la_environment::la_environment_current_user(out, error_out)
        })?))
    }

    /// Snapshot the current environment state.
    ///
    /// # Errors
    ///
    /// Returns an error if the macOS 15 environment APIs are unavailable or the Swift bridge rejects the request.
    pub fn state(&self) -> Result<LAEnvironmentState> {
        Ok(LAEnvironmentState::from_raw(bridge_ptr(
            |out, error_out| unsafe {
                ffi::la_environment::la_environment_get_state(self.handle.as_ptr(), out, error_out)
            },
        )?))
    }

    /// Register an observer for environment state changes.
    ///
    /// Keep the returned registration alive for as long as the observer should remain registered.
    /// Dropping it releases the weakly-held observer object.
    ///
    /// # Errors
    ///
    /// Returns an error if the macOS 15 environment APIs are unavailable or the Swift bridge rejects the request.
    pub fn add_observer<O>(&self, observer: O) -> Result<LAEnvironmentObserverRegistration>
    where
        O: LAEnvironmentObserver,
    {
        let context = Box::new(EnvironmentObserverContext {
            observer: Box::new(observer),
        });
        let context_ptr = Box::into_raw(context).cast::<c_void>();

        let observer_raw = match bridge_ptr(|out, error_out| unsafe {
            ffi::la_environment::la_environment_observer_new(
                Some(environment_observer_trampoline),
                Some(environment_observer_release),
                context_ptr,
                out,
                error_out,
            )
        }) {
            Ok(raw) => raw,
            Err(error) => {
                unsafe { environment_observer_release(context_ptr) };
                return Err(error);
            }
        };

        let registration = LAEnvironmentObserverRegistration {
            handle: OwnedHandle::new(
                observer_raw,
                ffi::la_environment::la_environment_observer_release,
            ),
        };

        if let Err(error) = bridge_unit(|error_out| unsafe {
            ffi::la_environment::la_environment_add_observer(
                self.handle.as_ptr(),
                registration.handle.as_ptr(),
                error_out,
            )
        }) {
            drop(registration);
            return Err(error);
        }

        Ok(registration)
    }

    /// Remove a previously registered observer.
    ///
    /// # Errors
    ///
    /// Returns an error if the macOS 15 environment APIs are unavailable or the Swift bridge rejects the request.
    pub fn remove_observer(&self, observer: &LAEnvironmentObserverRegistration) -> Result<()> {
        bridge_unit(|error_out| unsafe {
            ffi::la_environment::la_environment_remove_observer(
                self.handle.as_ptr(),
                observer.as_ptr(),
                error_out,
            )
        })
    }
}

/// Snapshot wrapper around Apple's `LAEnvironmentState`.
#[derive(Debug)]
pub struct LAEnvironmentState {
    handle: OwnedHandle,
}

impl LAEnvironmentState {
    pub(crate) fn from_raw(raw: NonNull<c_void>) -> Self {
        Self {
            handle: OwnedHandle::new(raw, ffi::la_environment::la_environment_state_release),
        }
    }

    #[allow(dead_code)]
    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    /// Information about the device's biometric mechanism, if supported.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn biometry(&self) -> Result<Option<LAEnvironmentMechanismBiometry>> {
        let raw = bridge_opt_ptr(|out, error_out| unsafe {
            ffi::la_environment::la_environment_state_get_biometry(self.handle.as_ptr(), out, error_out)
        })?;
        Ok(raw.map(LAEnvironmentMechanismBiometry::from_raw))
    }

    /// Information about the local user password or passcode mechanism, if supported.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn user_password(&self) -> Result<Option<LAEnvironmentMechanismUserPassword>> {
        let raw = bridge_opt_ptr(|out, error_out| unsafe {
            ffi::la_environment::la_environment_state_get_user_password(
                self.handle.as_ptr(),
                out,
                error_out,
            )
        })?;
        Ok(raw.map(LAEnvironmentMechanismUserPassword::from_raw))
    }

    /// Companion authentication mechanisms currently paired with this device.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn companions(&self) -> Result<Vec<LAEnvironmentMechanismCompanion>> {
        let count = count_to_usize(
            bridge_i64(|out, error_out| unsafe {
                ffi::la_environment::la_environment_state_get_companion_count(
                    self.handle.as_ptr(),
                    out,
                    error_out,
                )
            })?,
            "companion",
        )?;

        let mut mechanisms = Vec::with_capacity(count);
        for index in 0..count {
            let index = i64::try_from(index).map_err(|_| {
                LAError::BridgeFailed(
                    "LocalAuthentication returned more companion mechanisms than this platform can index"
                        .to_owned(),
                )
            })?;
            let raw = bridge_ptr(|out, error_out| unsafe {
                ffi::la_environment::la_environment_state_get_companion_at(
                    self.handle.as_ptr(),
                    index,
                    out,
                    error_out,
                )
            })?;
            mechanisms.push(LAEnvironmentMechanismCompanion::from_raw(raw));
        }
        Ok(mechanisms)
    }

    /// Information about every currently known authentication mechanism.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn all_mechanisms(&self) -> Result<Vec<LAEnvironmentMechanism>> {
        let count = count_to_usize(
            bridge_i64(|out, error_out| unsafe {
                ffi::la_environment::la_environment_state_get_all_mechanism_count(
                    self.handle.as_ptr(),
                    out,
                    error_out,
                )
            })?,
            "mechanism",
        )?;

        let mut mechanisms = Vec::with_capacity(count);
        for index in 0..count {
            let index = i64::try_from(index).map_err(|_| {
                LAError::BridgeFailed(
                    "LocalAuthentication returned more mechanisms than this platform can index"
                        .to_owned(),
                )
            })?;
            let raw = bridge_ptr(|out, error_out| unsafe {
                ffi::la_environment::la_environment_state_get_all_mechanism_at(
                    self.handle.as_ptr(),
                    index,
                    out,
                    error_out,
                )
            })?;
            mechanisms.push(LAEnvironmentMechanism::from_raw(raw));
        }
        Ok(mechanisms)
    }
}

/// Common properties shared by every `LAEnvironment` authentication mechanism.
#[derive(Debug)]
pub struct LAEnvironmentMechanism {
    handle: OwnedHandle,
}

impl LAEnvironmentMechanism {
    pub(crate) fn from_raw(raw: NonNull<c_void>) -> Self {
        Self {
            handle: OwnedHandle::new(raw, ffi::la_environment::la_environment_mechanism_release),
        }
    }

    #[allow(dead_code)]
    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    /// Whether the mechanism is currently usable for authentication.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn is_usable(&self) -> Result<bool> {
        mechanism_is_usable(self.handle.as_ptr())
    }

    /// Localized display name such as `Touch ID` or `Password`.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn localized_name(&self) -> Result<String> {
        mechanism_localized_name(self.handle.as_ptr())
    }

    /// SF Symbol name representing this mechanism.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn icon_system_name(&self) -> Result<String> {
        mechanism_icon_system_name(self.handle.as_ptr())
    }
}

/// Biometric `LAEnvironment` mechanism details.
#[derive(Debug)]
pub struct LAEnvironmentMechanismBiometry {
    handle: OwnedHandle,
}

impl LAEnvironmentMechanismBiometry {
    pub(crate) fn from_raw(raw: NonNull<c_void>) -> Self {
        Self {
            handle: OwnedHandle::new(raw, ffi::la_environment::la_environment_mechanism_release),
        }
    }

    #[allow(dead_code)]
    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    /// Whether the mechanism is currently usable for authentication.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn is_usable(&self) -> Result<bool> {
        mechanism_is_usable(self.handle.as_ptr())
    }

    /// Localized display name such as `Touch ID` or `Face ID`.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn localized_name(&self) -> Result<String> {
        mechanism_localized_name(self.handle.as_ptr())
    }

    /// SF Symbol name representing this mechanism.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn icon_system_name(&self) -> Result<String> {
        mechanism_icon_system_name(self.handle.as_ptr())
    }

    /// Hardware biometry type supported by the device.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn biometry_type(&self) -> Result<BiometryType> {
        Ok(BiometryType::from_ffi(bridge_i32(|out, error_out| unsafe {
            ffi::la_environment::la_environment_mechanism_biometry_get_biometry_type(
                self.handle.as_ptr(),
                out,
                error_out,
            )
        })?))
    }

    /// Whether the user has enrolled this biometric mechanism.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn is_enrolled(&self) -> Result<bool> {
        bridge_bool(|out, error_out| unsafe {
            ffi::la_environment::la_environment_mechanism_biometry_get_is_enrolled(
                self.handle.as_ptr(),
                out,
                error_out,
            )
        })
    }

    /// Whether the biometric mechanism is locked out.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn is_locked_out(&self) -> Result<bool> {
        bridge_bool(|out, error_out| unsafe {
            ffi::la_environment::la_environment_mechanism_biometry_get_is_locked_out(
                self.handle.as_ptr(),
                out,
                error_out,
            )
        })
    }

    /// Application-specific biometric enrollment hash.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn state_hash(&self) -> Result<Vec<u8>> {
        bridge_bytes(|out, out_len, error_out| unsafe {
            ffi::la_environment::la_environment_mechanism_biometry_get_state_hash(
                self.handle.as_ptr(),
                out,
                out_len,
                error_out,
            )
        })
    }

    /// Whether the built-in biometric sensor is inaccessible.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn built_in_sensor_inaccessible(&self) -> Result<bool> {
        bridge_bool(|out, error_out| unsafe {
            ffi::la_environment::la_environment_mechanism_biometry_get_built_in_sensor_inaccessible(
                self.handle.as_ptr(),
                out,
                error_out,
            )
        })
    }
}

/// Companion-device `LAEnvironment` mechanism details.
#[derive(Debug)]
pub struct LAEnvironmentMechanismCompanion {
    handle: OwnedHandle,
}

impl LAEnvironmentMechanismCompanion {
    pub(crate) fn from_raw(raw: NonNull<c_void>) -> Self {
        Self {
            handle: OwnedHandle::new(raw, ffi::la_environment::la_environment_mechanism_release),
        }
    }

    #[allow(dead_code)]
    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    /// Whether the mechanism is currently usable for authentication.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn is_usable(&self) -> Result<bool> {
        mechanism_is_usable(self.handle.as_ptr())
    }

    /// Localized display name such as `Apple Watch`.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn localized_name(&self) -> Result<String> {
        mechanism_localized_name(self.handle.as_ptr())
    }

    /// SF Symbol name representing this mechanism.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn icon_system_name(&self) -> Result<String> {
        mechanism_icon_system_name(self.handle.as_ptr())
    }

    /// Companion-device type.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn companion_type(&self) -> Result<LACompanionType> {
        Ok(LACompanionType::from_ffi(bridge_i32(|out, error_out| unsafe {
            ffi::la_environment::la_environment_mechanism_companion_get_type(
                self.handle.as_ptr(),
                out,
                error_out,
            )
        })?))
    }

    /// Pairing hash for the current companion type, if one exists.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn state_hash(&self) -> Result<Option<Vec<u8>>> {
        bridge_opt_bytes(|out, out_len, error_out| unsafe {
            ffi::la_environment::la_environment_mechanism_companion_get_state_hash(
                self.handle.as_ptr(),
                out,
                out_len,
                error_out,
            )
        })
    }
}

/// Password or passcode `LAEnvironment` mechanism details.
#[derive(Debug)]
pub struct LAEnvironmentMechanismUserPassword {
    handle: OwnedHandle,
}

impl LAEnvironmentMechanismUserPassword {
    pub(crate) fn from_raw(raw: NonNull<c_void>) -> Self {
        Self {
            handle: OwnedHandle::new(raw, ffi::la_environment::la_environment_mechanism_release),
        }
    }

    #[allow(dead_code)]
    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    /// Whether the mechanism is currently usable for authentication.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn is_usable(&self) -> Result<bool> {
        mechanism_is_usable(self.handle.as_ptr())
    }

    /// Localized display name such as `Password`.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn localized_name(&self) -> Result<String> {
        mechanism_localized_name(self.handle.as_ptr())
    }

    /// SF Symbol name representing this mechanism.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn icon_system_name(&self) -> Result<String> {
        mechanism_icon_system_name(self.handle.as_ptr())
    }

    /// Whether the local user password or passcode is set.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge rejects the request.
    pub fn is_set(&self) -> Result<bool> {
        bridge_bool(|out, error_out| unsafe {
            ffi::la_environment::la_environment_mechanism_user_password_get_is_set(
                self.handle.as_ptr(),
                out,
                error_out,
            )
        })
    }
}
