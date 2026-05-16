use core::ffi::{c_char, c_void};

pub type la_environment_observer_callback = Option<
    unsafe extern "C" fn(
        context: *mut c_void,
        environment: *mut c_void,
        old_state: *mut c_void,
    ),
>;
pub type la_environment_observer_release =
    Option<unsafe extern "C" fn(context: *mut c_void)>;

extern "C" {
    pub fn la_environment_current_user(
        out_environment: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_environment_release(environment: *mut c_void);
    pub fn la_environment_get_state(
        environment: *mut c_void,
        out_state: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_environment_add_observer(
        environment: *mut c_void,
        observer: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_environment_remove_observer(
        environment: *mut c_void,
        observer: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;

    pub fn la_environment_observer_new(
        callback: la_environment_observer_callback,
        release: la_environment_observer_release,
        context: *mut c_void,
        out_observer: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_environment_observer_release(observer: *mut c_void);

    pub fn la_environment_state_release(state: *mut c_void);
    pub fn la_environment_state_get_biometry(
        state: *mut c_void,
        out_mechanism: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_environment_state_get_user_password(
        state: *mut c_void,
        out_mechanism: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_environment_state_get_companion_count(
        state: *mut c_void,
        out_count: *mut i64,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_environment_state_get_companion_at(
        state: *mut c_void,
        index: i64,
        out_mechanism: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_environment_state_get_all_mechanism_count(
        state: *mut c_void,
        out_count: *mut i64,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_environment_state_get_all_mechanism_at(
        state: *mut c_void,
        index: i64,
        out_mechanism: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;

    pub fn la_environment_mechanism_release(mechanism: *mut c_void);
    pub fn la_environment_mechanism_get_is_usable(
        mechanism: *mut c_void,
        out_value: *mut u8,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_environment_mechanism_get_localized_name(
        mechanism: *mut c_void,
        out_value: *mut *mut c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_environment_mechanism_get_icon_system_name(
        mechanism: *mut c_void,
        out_value: *mut *mut c_char,
        error_out: *mut *mut c_char,
    ) -> i32;

    pub fn la_environment_mechanism_biometry_get_biometry_type(
        mechanism: *mut c_void,
        out_value: *mut i32,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_environment_mechanism_biometry_get_is_enrolled(
        mechanism: *mut c_void,
        out_value: *mut u8,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_environment_mechanism_biometry_get_is_locked_out(
        mechanism: *mut c_void,
        out_value: *mut u8,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_environment_mechanism_biometry_get_state_hash(
        mechanism: *mut c_void,
        out_bytes: *mut *mut u8,
        out_len: *mut usize,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_environment_mechanism_biometry_get_built_in_sensor_inaccessible(
        mechanism: *mut c_void,
        out_value: *mut u8,
        error_out: *mut *mut c_char,
    ) -> i32;

    pub fn la_environment_mechanism_companion_get_type(
        mechanism: *mut c_void,
        out_value: *mut i32,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn la_environment_mechanism_companion_get_state_hash(
        mechanism: *mut c_void,
        out_bytes: *mut *mut u8,
        out_len: *mut usize,
        error_out: *mut *mut c_char,
    ) -> i32;

    pub fn la_environment_mechanism_user_password_get_is_set(
        mechanism: *mut c_void,
        out_value: *mut u8,
        error_out: *mut *mut c_char,
    ) -> i32;
}
