use crate::audio::audio_config::AudioConfig;
use crate::profile::monitor_config::MonitorConfig;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub id: Option<String>,
    pub name: String,
    pub monitors_config: Vec<MonitorConfig>,
    pub audio_sinks_config: Vec<AudioConfig>,
    pub initial_to_monitors_delay_ms: Option<u32>,
    pub monitors_to_audio_delay_ms: Option<u32>,
    pub audio_to_desktop_delay_ms: Option<u32>,
}
