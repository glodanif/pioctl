use crate::display::display_error::DisplayError;
use crate::display::display_manager::DisplayManager;
use crate::profile::profile::Profile;
use std::fs;
use std::path::PathBuf;

pub struct ProfilesManager<'a> {
    _display_manager: &'a Box<dyn DisplayManager>,
    config_dir: PathBuf,
    data_dir: PathBuf,
}

impl<'a> ProfilesManager<'a> {
    pub fn new(display_manager: &'a Box<dyn DisplayManager>) -> Self {
        let config_dir = dirs::config_dir()
            .expect("Could not find config directory")
            .join(env!("CARGO_PKG_NAME"));
        let data_dir = dirs::data_dir()
            .expect("Could not find data directory")
            .join(env!("CARGO_PKG_NAME"));
        ProfilesManager {
            _display_manager: display_manager,
            config_dir,
            data_dir,
        }
    }

    // Returns (filename_stem, Profile) pairs sorted by stem.
    fn load_profiles(&self) -> Result<Vec<(String, Profile)>, DisplayError> {
        fs::create_dir_all(&self.config_dir).map_err(|_| DisplayError::FailedToCreateConfig)?;
        let mut profiles = Vec::new();
        for entry in fs::read_dir(&self.config_dir).map_err(|_| DisplayError::FailedToGetConfig)? {
            let entry = entry.map_err(|_| DisplayError::FailedToGetConfig)?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                let stem = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string();
                let content =
                    fs::read_to_string(&path).map_err(|_| DisplayError::FailedToGetConfig)?;
                let profile: Profile = serde_json::from_str(&content).map_err(|e| {
                    println!("Failed to parse profile {:?}: {}", path, e);
                    DisplayError::FailedToGetConfig
                })?;
                profiles.push((stem, profile));
            }
        }
        profiles.sort_by(|a, b| a.0.cmp(&b.0));
        Ok(profiles)
    }

    fn get_current_profile_stem(&self) -> Result<String, DisplayError> {
        let path = self.data_dir.join("current_profile");
        if path.exists() {
            fs::read_to_string(&path)
                .map(|s| s.trim().to_string())
                .map_err(|_| DisplayError::FailedToGetConfig)
        } else {
            Err(DisplayError::CurrentProfileNotSet)
        }
    }

    fn save_current_profile_stem(&self, stem: &str) -> Result<(), DisplayError> {
        fs::create_dir_all(&self.data_dir).map_err(|_| DisplayError::FailedToSetConfig)?;
        fs::write(self.data_dir.join("current_profile"), stem)
            .map_err(|_| DisplayError::FailedToSetConfig)
    }

    fn get_profile_by_stem(&self, stem: &str) -> Result<Profile, DisplayError> {
        let path = self.config_dir.join(format!("{}.json", stem));
        if !path.exists() {
            return Err(DisplayError::ProfileNotFound);
        }
        let content = fs::read_to_string(&path).map_err(|_| DisplayError::FailedToGetConfig)?;
        serde_json::from_str(&content).map_err(|_| DisplayError::FailedToGetConfig)
    }

    pub fn get_profiles(&self) -> Result<String, DisplayError> {
        let profiles = self.load_profiles()?;
        let mut parts: Vec<String> = Vec::new();
        for (stem, profile) in profiles {
            let path = self.config_dir.join(format!("{}.json", stem));
            let content = serde_json::to_string_pretty(&profile)
                .map_err(|_| DisplayError::EncodingError("get_profiles"))?;
            parts.push(format!("{}", path.display()) + "\n" + &content);
        }
        Ok(parts.join("\n---\n"))
    }

    pub fn get_current_profile(&self) -> Result<Profile, DisplayError> {
        match self.get_current_profile_stem() {
            Ok(stem) => self.get_profile_by_stem(&stem),
            Err(DisplayError::CurrentProfileNotSet) => {
                let mut profiles = self.load_profiles()?;
                if profiles.is_empty() {
                    return Err(DisplayError::ProfileNotFound);
                }
                let (stem, profile) = profiles.remove(0);
                self.save_current_profile_stem(&stem)?;
                Ok(profile)
            }
            Err(err) => Err(err),
        }
    }

    pub fn get_current_profile_json(&self) -> Result<String, DisplayError> {
        let stem = match self.get_current_profile_stem() {
            Ok(stem) => stem,
            Err(DisplayError::CurrentProfileNotSet) => {
                let mut profiles = self.load_profiles()?;
                if profiles.is_empty() {
                    return Err(DisplayError::ProfileNotFound);
                }
                let (stem, _) = profiles.remove(0);
                self.save_current_profile_stem(&stem)?;
                stem
            }
            Err(err) => return Err(err),
        };
        let path = self.config_dir.join(format!("{}.json", stem));
        let profile = self.get_profile_by_stem(&stem)?;
        let content = serde_json::to_string_pretty(&profile)
            .map_err(|_| DisplayError::EncodingError("get_current_profile_json"))?;
        Ok(format!("{}", path.display()) + "\n" + &content)
    }

    pub fn get_profile_by_id(&self, id: String) -> Result<Profile, DisplayError> {
        self.get_profile_by_stem(&id)
    }

    pub fn get_next_profile(&self) -> Result<Profile, DisplayError> {
        let profiles = self.load_profiles()?;
        if profiles.len() < 2 {
            return Err(DisplayError::NotEnoughProfiles);
        }

        self.get_current_profile()?;
        let current_stem = self.get_current_profile_stem()?;

        let current_index = profiles
            .iter()
            .position(|(stem, _)| *stem == current_stem)
            .ok_or(DisplayError::ProfileNotFound)?;
        let next_index = (current_index + 1) % profiles.len();

        Ok(profiles.into_iter().nth(next_index).unwrap().1)
    }

    // Accepts the filename stem (e.g. "pc", "tv") as the profile id.
    pub fn set_current_profile_id(&self, profile_id: String) -> Result<(), DisplayError> {
        let path = self.config_dir.join(format!("{}.json", profile_id));
        if !path.exists() {
            // Fall back: search by display name so callers passing profile.name still work.
            let profiles = self.load_profiles()?;
            let stem = profiles
                .iter()
                .find(|(_, p)| p.name == profile_id)
                .map(|(s, _)| s.clone())
                .ok_or(DisplayError::ProfileNotFound)?;
            return self.save_current_profile_stem(&stem);
        }
        self.save_current_profile_stem(&profile_id)
    }
}
