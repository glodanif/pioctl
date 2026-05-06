use crate::audio::audio_error::AudioError;
use crate::profile::Profile;

pub trait AudioManager {
    fn get_audio_sinks(&self, dry_run: bool) -> Result<Vec<String>, AudioError>;
    fn set_audio_sinks(&self, profile: &Profile, dry_run: bool) -> Result<(), AudioError>;
}
