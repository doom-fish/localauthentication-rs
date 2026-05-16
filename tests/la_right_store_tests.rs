mod common;

use localauthentication::prelude::*;

#[test]
fn right_store_shared_and_save_path_are_reachable() -> Result<(), Box<dyn std::error::Error>> {
    let store = LARightStore::shared()?;
    let right = LARight::new()?;
    let identifier = common::unique_identifier("store-test");

    match store.save_right(&right, &identifier) {
        Ok(persisted) => {
            let fetched = store.right_for_identifier(&identifier)?;
            assert_eq!(persisted.state()?, fetched.state()?);
            store.remove_right(&fetched)?;
        }
        Err(error) => {
            assert_ne!(error.code(), 0);
            assert!(!error.message().is_empty());
        }
    }

    Ok(())
}
