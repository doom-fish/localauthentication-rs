//! `LAPolicy` values supported by `LAContext`.

use crate::ffi;

/// Authentication policies supported on macOS by `LocalAuthentication.framework`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LAPolicy {
    /// Authenticate the device owner using biometry only.
    DeviceOwnerAuthenticationWithBiometrics,
    /// Authenticate the device owner using biometry or the local password.
    DeviceOwnerAuthentication,
    /// Authenticate the device owner using a nearby companion device.
    DeviceOwnerAuthenticationWithCompanion,
    /// Authenticate the device owner using biometry or a nearby companion device.
    DeviceOwnerAuthenticationWithBiometricsOrCompanion,
    /// Deprecated alias for companion-device authentication.
    #[deprecated(note = "Use `DeviceOwnerAuthenticationWithCompanion` instead.")]
    DeviceOwnerAuthenticationWithWatch,
    /// Deprecated alias for biometry-or-companion authentication.
    #[deprecated(note = "Use `DeviceOwnerAuthenticationWithBiometricsOrCompanion` instead.")]
    DeviceOwnerAuthenticationWithBiometricsOrWatch,
}

/// Backward-compatible alias for the v0.1.x enum name.
pub type Policy = LAPolicy;

impl LAPolicy {
    #[allow(deprecated)]
    #[must_use]
    pub const fn raw_value(self) -> i32 {
        match self {
            Self::DeviceOwnerAuthenticationWithBiometrics => {
                ffi::la_policy::DEVICE_OWNER_AUTHENTICATION_WITH_BIOMETRICS
            }
            Self::DeviceOwnerAuthentication => ffi::la_policy::DEVICE_OWNER_AUTHENTICATION,
            Self::DeviceOwnerAuthenticationWithCompanion => {
                ffi::la_policy::DEVICE_OWNER_AUTHENTICATION_WITH_COMPANION
            }
            Self::DeviceOwnerAuthenticationWithBiometricsOrCompanion => {
                ffi::la_policy::DEVICE_OWNER_AUTHENTICATION_WITH_BIOMETRICS_OR_COMPANION
            }
            Self::DeviceOwnerAuthenticationWithWatch => {
                ffi::la_policy::DEVICE_OWNER_AUTHENTICATION_WITH_WATCH
            }
            Self::DeviceOwnerAuthenticationWithBiometricsOrWatch => {
                ffi::la_policy::DEVICE_OWNER_AUTHENTICATION_WITH_BIOMETRICS_OR_WATCH
            }
        }
    }

    pub(crate) const fn as_ffi(self) -> i32 {
        self.raw_value()
    }

    /// A short, human-readable name for the policy.
    #[allow(deprecated)]
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::DeviceOwnerAuthenticationWithBiometrics => {
                "device owner authentication with biometrics"
            }
            Self::DeviceOwnerAuthentication => "device owner authentication",
            Self::DeviceOwnerAuthenticationWithCompanion => {
                "device owner authentication with companion"
            }
            Self::DeviceOwnerAuthenticationWithBiometricsOrCompanion => {
                "device owner authentication with biometrics or companion"
            }
            Self::DeviceOwnerAuthenticationWithWatch => "device owner authentication with watch",
            Self::DeviceOwnerAuthenticationWithBiometricsOrWatch => {
                "device owner authentication with biometrics or watch"
            }
        }
    }
}
