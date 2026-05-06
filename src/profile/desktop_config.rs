use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DesktopConfig {
    pub delay_before_ms: Option<u32>,
}
