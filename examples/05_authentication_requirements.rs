use localauthentication::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let default_requirement = LAAuthenticationRequirement::default_requirement()?;
    let biometry_requirement = LAAuthenticationRequirement::biometry_requirement()?;
    let current_set_requirement = LAAuthenticationRequirement::biometry_current_set_requirement()?;
    let fallback = LABiometryFallbackRequirement::device_passcode_requirement()?;
    let fallback_requirement =
        LAAuthenticationRequirement::biometry_requirement_with_fallback(&fallback)?;

    let rights = [
        LARight::new_with_requirement(&default_requirement)?,
        LARight::new_with_requirement(&biometry_requirement)?,
        LARight::new_with_requirement(&current_set_requirement)?,
        LARight::new_with_requirement(&fallback_requirement)?,
    ];

    for (index, right) in rights.iter().enumerate() {
        println!("right {index} state: {:?}", right.state()?);
        println!("right {index} preflight: {:?}", right.check_can_authorize());
    }

    println!("✅ authentication requirements OK");
    Ok(())
}
