use std::time::{SystemTime, UNIX_EPOCH};

use localauthentication::LARightStore;

pub fn unique_identifier(prefix: &str) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("dev.doomfish.localauthentication.{prefix}.{now}")
}

pub struct RemoveOnDrop<'a> {
    store: &'a LARightStore,
    identifier: Option<&'a str>,
}

impl<'a> RemoveOnDrop<'a> {
    pub const fn new(store: &'a LARightStore, identifier: &'a str) -> Self {
        Self {
            store,
            identifier: Some(identifier),
        }
    }

    pub fn disarm(mut self) {
        self.identifier = None;
    }
}

impl Drop for RemoveOnDrop<'_> {
    fn drop(&mut self) {
        if let Some(identifier) = self.identifier {
            if let Err(error) = self.store.remove_right_for_identifier(identifier) {
                eprintln!("failed to remove the test right {identifier}: {error}");
            }
        }
    }
}
