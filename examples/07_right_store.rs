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
    let identifier = unique_identifier("store");

    match store.save_right(&right, &identifier) {
        Ok(persisted) => {
            println!("saved state: {:?}", persisted.state()?);
            store.remove_right(&persisted)?;
        }
        Err(error) => {
            println!("save_right expectedly failed in unsigned/headless environments: {error}");
        }
    }

    println!("✅ right-store smoke OK");
    Ok(())
}
