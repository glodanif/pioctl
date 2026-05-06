use crate::profile::monitor_config::MonitorConfig;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MonitorsConfig {
    pub delay_before_ms: Option<u32>,
    pub disabled_to_enabled_delay_ms: Option<u32>,
    pub monitors: Vec<MonitorConfig>,
}
