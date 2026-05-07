use crate::profile::audio_sinks_config::AudioSinksConfig;
use crate::profile::desktop_config::DesktopConfig;
use crate::profile::monitors_config::MonitorsConfig;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub monitors_config: MonitorsConfig,
    pub audio_sinks_config: AudioSinksConfig,
    pub desktop_config: DesktopConfig,
}
