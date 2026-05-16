mod common;

use localauthentication::prelude::*;

#[test]
fn persisted_right_secret_and_tag_are_accessible_when_storage_succeeds(
) -> Result<(), Box<dyn std::error::Error>> {
    let store = LARightStore::shared()?;
    let right = LARight::new()?;
    let identifier = common::unique_identifier("persisted-test");

    let persisted = match store.save_right_with_secret(&right, &identifier, b"top-secret") {
        Ok(persisted) => persisted,
        Err(error) => {
            eprintln!("skipping live persisted-right assertions: {error}");
            return Ok(());
        }
    };

    persisted.set_tag(123)?;
    assert_eq!(persisted.tag()?, 123);
    assert_eq!(persisted.secret()?.load_data()?, b"top-secret");
    let _ = persisted.check_can_authorize();
    store.remove_right(&persisted)?;
    Ok(())
}
