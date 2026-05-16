use localauthentication::prelude::*;

#[test]
fn right_tag_state_and_preflight_are_accessible() -> Result<(), Box<dyn std::error::Error>> {
    let right = LARight::new()?;
    right.set_tag(99)?;

    assert_eq!(right.tag()?, 99);
    let _ = right.state()?;
    let _ = right.check_can_authorize();
    right.deauthorize()?;
    Ok(())
}
