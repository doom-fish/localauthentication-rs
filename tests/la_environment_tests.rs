use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use localauthentication::prelude::*;

struct CounterObserver {
    changes: Arc<AtomicUsize>,
}

impl LAEnvironmentObserver for CounterObserver {
    fn state_did_change(&self, environment: &LAEnvironment, old_state: &LAEnvironmentState) {
        let _ = environment.state();
        let _ = old_state.all_mechanisms();
        self.changes.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
fn environment_state_and_observer_registration_are_reachable(
) -> Result<(), Box<dyn std::error::Error>> {
    let environment = match LAEnvironment::current_user() {
        Ok(environment) => environment,
        Err(error) if error.message().contains("macOS 15.0") => {
            eprintln!("skipping macOS 15-only environment assertions: {error}");
            return Ok(());
        }
        Err(error) => return Err(Box::new(error)),
    };

    let changes = Arc::new(AtomicUsize::new(0));
    let registration = environment.add_observer(CounterObserver {
        changes: Arc::clone(&changes),
    })?;

    let state = environment.state()?;
    for mechanism in state.all_mechanisms()? {
        let _ = mechanism.is_usable()?;
        assert!(!mechanism.localized_name()?.is_empty());
        assert!(!mechanism.icon_system_name()?.is_empty());
    }

    if let Some(biometry) = state.biometry()? {
        let _ = biometry.is_usable()?;
        let _ = biometry.localized_name()?;
        let _ = biometry.icon_system_name()?;
        let _ = biometry.biometry_type()?;
        let _ = biometry.is_enrolled()?;
        let _ = biometry.is_locked_out()?;
        let _ = biometry.state_hash()?;
        let _ = biometry.built_in_sensor_inaccessible()?;
    }

    if let Some(user_password) = state.user_password()? {
        let _ = user_password.is_usable()?;
        let _ = user_password.localized_name()?;
        let _ = user_password.icon_system_name()?;
        let _ = user_password.is_set()?;
    }

    for companion in state.companions()? {
        let _ = companion.is_usable()?;
        let _ = companion.localized_name()?;
        let _ = companion.icon_system_name()?;
        let _ = companion.companion_type()?;
        let _ = companion.state_hash()?;
    }

    environment.remove_observer(&registration)?;
    assert_eq!(changes.load(Ordering::Relaxed), 0);

    Ok(())
}
