use crate::audio::audio_config::AudioConfig;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AudioSinksConfig {
    pub delay_before_ms: Option<u32>,
    pub audio_sinks: Vec<AudioConfig>,
}
