use crate::audio::audio_error::AudioError;
use crate::profile::audio_sinks_config::AudioSinksConfig;

pub trait AudioManager {
    fn get_audio_sinks(&self, dry_run: bool) -> Result<Vec<String>, AudioError>;
    fn set_audio_sinks_config(&self, config: &AudioSinksConfig, dry_run: bool) -> Result<(), AudioError>;
}
