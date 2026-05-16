use std::time::{SystemTime, UNIX_EPOCH};

pub fn unique_identifier(prefix: &str) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("dev.doomfish.localauthentication.{prefix}.{now}")
}
