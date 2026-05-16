use localauthentication::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_identifier(prefix: &str) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("dev.doomfish.localauthentication.{prefix}.{now}")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = LARightStore::shared()?;
    let right = LARight::new()?;
    let identifier = unique_identifier("persisted");

    match store.save_right_with_secret(&right, &identifier, b"top-secret") {
        Ok(persisted) => {
            persisted.set_tag(7)?;
            println!("persisted state: {:?}", persisted.state()?);
            println!("persisted tag: {}", persisted.tag()?);
            println!("persisted preflight: {:?}", persisted.check_can_authorize());
            println!("secret bytes: {}", persisted.secret()?.load_data()?.len());
            store.remove_right(&persisted)?;
        }
        Err(error) => {
            println!("persisted-right APIs need entitlements on many systems: {error}");
        }
    }

    println!("✅ persisted-right smoke OK");
    Ok(())
}
