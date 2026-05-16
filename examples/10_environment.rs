use localauthentication::prelude::*;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let environment = match LAEnvironment::current_user() {
        Ok(environment) => environment,
        Err(error) => {
            println!("environment APIs require macOS 15+: {error}");
            return Ok(());
        }
    };

    let notifications = Arc::new(AtomicUsize::new(0));
    let registration = environment.add_observer({
        let notifications = Arc::clone(&notifications);
        move |environment: &LAEnvironment, old_state: &LAEnvironmentState| {
            let current_count = environment
                .state()
                .and_then(|state| state.all_mechanisms().map(|items| items.len()))
                .unwrap_or_default();
            let previous_count = old_state
                .all_mechanisms()
                .map(|items| items.len())
                .unwrap_or_default();
            println!(
                "environment changed: {previous_count} -> {current_count} mechanisms"
            );
            notifications.fetch_add(1, Ordering::Relaxed);
        }
    })?;

    let state = environment.state()?;
    println!("all mechanisms: {}", state.all_mechanisms()?.len());

    if let Some(biometry) = state.biometry()? {
        println!(
            "biometry: {:?}, enrolled={}, usable={}, icon={}",
            biometry.biometry_type()?,
            biometry.is_enrolled()?,
            biometry.is_usable()?,
            biometry.icon_system_name()?
        );
    }

    if let Some(user_password) = state.user_password()? {
        println!(
            "user password: set={}, usable={}, label={}",
            user_password.is_set()?,
            user_password.is_usable()?,
            user_password.localized_name()?
        );
    }

    for companion in state.companions()? {
        println!(
            "companion {:?}: usable={}, state-hash-bytes={}",
            companion.companion_type()?,
            companion.is_usable()?,
            companion.state_hash()?.map_or(0, |hash| hash.len())
        );
    }

    environment.remove_observer(&registration)?;
    println!(
        "observer notifications seen so far: {}",
        notifications.load(Ordering::Relaxed)
    );
    println!("✅ environment smoke OK");
    Ok(())
}
