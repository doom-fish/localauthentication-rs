use localauthentication::ffi;
use localauthentication::prelude::*;

fn main() {
    let errors = [
        LAError::from_code_message(ffi::la_error::BIOMETRY_LOCKOUT, "biometry locked"),
        LAError::from_code_message(ffi::la_error::COMPANION_NOT_AVAILABLE, "companion missing"),
        LAError::from_code_message(ffi::status::BRIDGE_FAILED, "bridge failed"),
    ];

    println!("domain: {LA_ERROR_DOMAIN}");
    for error in errors {
        println!("{error:?} => {error}");
    }
    println!("✅ error mapping OK");
}
