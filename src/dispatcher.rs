use crate::audio::audio_manager::AudioManager;
use crate::cli::Command;
use crate::display::desktop_manager::DesktopManager;
use crate::display::display_manager::DisplayManager;
use crate::notifications::notifications_manager::NotificationsManager;
use crate::profile::{Profile, ProfilesManager};
use std::thread;
use std::time::Duration;

pub struct Dispatcher<'a> {
    dry_run: bool,
    display_manager: &'a Box<dyn DisplayManager>,
    audio_manager: &'a Box<dyn AudioManager>,
    desktop_manager: &'a Box<dyn DesktopManager>,
    profiles_manager: &'a ProfilesManager<'a>,
    notifications_manager: &'a NotificationsManager,
}

impl<'a> Dispatcher<'a> {
    pub fn new(
        dry_run: bool,
        display_manager: &'a Box<dyn DisplayManager>,
        audio_manager: &'a Box<dyn AudioManager>,
        desktop_manager: &'a Box<dyn DesktopManager>,
        profiles_manager: &'a ProfilesManager<'a>,
        notifications_manager: &'a NotificationsManager,
    ) -> Self {
        Self {
            dry_run,
            display_manager,
            audio_manager,
            desktop_manager,
            profiles_manager,
            notifications_manager,
        }
    }

    pub fn handle_command(&self, command: Option<Command>) {
        match command {
            Some(Command::Profiles) => {
                let profiles = &self.profiles_manager.get_profiles_json();
                match profiles {
                    Ok(profiles) => println!("{}", profiles),
                    Err(err) => eprintln!("Failed to get profiles: {}", err),
                }
            }
            Some(Command::AddProfile { profile_json }) => {
                let id = &self
                    .profiles_manager
                    .add_profile(profile_json, self.dry_run);
                match id {
                    Ok(id) => println!("Profile added successfully: {}", id),
                    Err(err) => eprintln!("Failed to add profile: {}", err),
                }
            }
            Some(Command::RemoveProfile { profile_id }) => {
                eprintln!("Remove profile not implemented yet: {}", profile_id);
            }
            Some(Command::Current) => {
                let current_profile = &self.profiles_manager.get_current_profile_json();
                match current_profile {
                    Ok(profile) => println!("{}", profile),
                    Err(err) => eprintln!("Failed to get current profile: {}", err),
                }
            }
            Some(Command::Restore { delay_ms }) => {
                match &self.profiles_manager.get_current_profile() {
                    Ok(profile) => {
                        let delay = delay_ms.unwrap_or(0);
                        if self.dry_run {
                            println!(
                                "[DRY RUN] Restoring profile: {}, with delay: {}ms",
                                profile.name, delay
                            );
                        } else {
                            thread::sleep(Duration::from_millis(delay));
                        }
                        self.apply_profile(profile)
                    }
                    Err(err) => eprintln!("Failed to get current profile: {}", err),
                }
            }
            Some(Command::Apply { profile_id }) => {
                match &self.profiles_manager.get_profile_by_id(profile_id) {
                    Ok(profile) => self.apply_profile(profile),
                    Err(err) => eprintln!("Failed to get profile by id: {}", err),
                }
            }
            Some(Command::ApplyNext) => match &self.profiles_manager.get_next_profile() {
                Ok(profile) => self.apply_profile(profile),
                Err(err) => eprintln!("Failed to get next profile: {}", err),
            },
            Some(Command::Monitors) => {
                let monitors = &self.display_manager.get_monitors_json(self.dry_run);
                match monitors {
                    Ok(monitors) => println!("{}", monitors),
                    Err(err) => eprintln!("Failed to get monitors: {}", err),
                }
            }
            Some(Command::AudioSinks) => {
                let audio_sinks = &self.audio_manager.get_audio_sinks(self.dry_run);
                match audio_sinks {
                    Ok(audio_sinks) => {
                        audio_sinks.iter().for_each(|sink| {
                            println!("{}", sink);
                        });
                    }
                    Err(err) => eprintln!("Failed to get audio sinks: {}", err),
                }
            }
            None => {
                eprintln!("No command provided, use --help for usage");
                std::process::exit(1);
            }
        }
    }

    fn apply_profile(&self, profile: &Profile) {
        const ICON_LOADING: &str = "\u{F110}";
        const ICON_CHECK: &str = "\u{F00C}";
        const ICON_ERROR: &str = "\u{F00D}";
        const ICON_SEP: &str = "\u{2003}";

        let title = format!("Applying profile: {}\n\u{00A0}", profile.name);
        let monitors_pending = format!("{}{}Monitors", ICON_LOADING, ICON_SEP);
        let audio_pending = format!("{}{}Audio", ICON_LOADING, ICON_SEP);
        let desktop_pending = format!("{}{}Desktop", ICON_LOADING, ICON_SEP);

        let notification_id = self
            .notifications_manager
            .notify_update(
                &title,
                &format!(
                    "{}\n{}\n{}",
                    monitors_pending, audio_pending, desktop_pending
                ),
                None,
                None,
                self.dry_run,
            )
            .ok();

        if let Some(delay) = profile.monitors_config.delay_before_ms {
            if self.dry_run {
                println!(
                    "[DRY RUN] Delay before setting monitors config: {}ms",
                    delay
                );
            } else {
                thread::sleep(Duration::from_millis(delay as u64));
            }
        }

        let monitors_result = self
            .display_manager
            .set_monitors_config(&profile.monitors_config, self.dry_run);
        let monitors_icon = if monitors_result.is_ok() {
            ICON_CHECK
        } else {
            ICON_ERROR
        };
        let monitors_done = format!("{}{}Monitors", monitors_icon, ICON_SEP);

        match monitors_result {
            Ok(_) => {
                if self.dry_run {
                    println!(
                        "[DRY RUN] Monitor config applied successfully: {}",
                        profile.name
                    );
                } else {
                    println!("Monitor config applied successfully: {}", profile.name);
                }
                if let Some(profile_id) = &profile.id {
                    if let Err(err) = self
                        .profiles_manager
                        .set_current_profile_id(profile_id.clone())
                    {
                        eprintln!("Warning: Failed to update monitors: {}", err);
                    }
                }
            }
            Err(err) => eprintln!("Failed to apply profile: {}", err),
        }

        let _ = self.notifications_manager.notify_update(
            &title,
            &format!("{}\n{}\n{}", monitors_done, audio_pending, desktop_pending),
            notification_id,
            None,
            self.dry_run,
        );

        if let Some(delay) = profile.audio_sinks_config.delay_before_ms {
            if self.dry_run {
                println!(
                    "[DRY RUN] Delay before settings audio sinks config: {}ms",
                    delay
                );
            } else {
                thread::sleep(Duration::from_millis(delay as u64));
            }
        }

        let audio_result = self
            .audio_manager
            .set_audio_sinks_config(&profile.audio_sinks_config, self.dry_run);
        let audio_icon = if audio_result.is_ok() {
            ICON_CHECK
        } else {
            ICON_ERROR
        };
        let audio_done = format!("{}{}Audio", audio_icon, ICON_SEP);

        match audio_result {
            Ok(_) => {
                if self.dry_run {
                    println!(
                        "[DRY RUN] Audio config applied successfully: {}",
                        profile.name
                    );
                } else {
                    println!("Audio config applied successfully: {}", profile.name);
                }
            }
            Err(err) => {
                eprintln!("Warning: Failed to update audio sinks {}", err);
            }
        }

        let _ = self.notifications_manager.notify_update(
            &title,
            &format!("{}\n{}\n{}", monitors_done, audio_done, desktop_pending),
            notification_id,
            None,
            self.dry_run,
        );

        if let Some(delay) = profile.desktop_config.delay_before_ms {
            if self.dry_run {
                println!("[DRY RUN] Delay before setting desktop config: {}ms", delay);
            } else {
                thread::sleep(Duration::from_millis(delay as u64));
            }
        }

        let desktop_result = self
            .desktop_manager
            .dispatch_desktops(&profile.desktop_config, self.dry_run);
        let desktop_icon = if desktop_result.is_ok() {
            ICON_CHECK
        } else {
            ICON_ERROR
        };
        let desktop_done = format!("{}{}Desktop", desktop_icon, ICON_SEP);

        match desktop_result {
            Ok(_) => {
                if self.dry_run {
                    println!(
                        "[DRY RUN] Desktop config applied successfully: {}",
                        profile.name
                    );
                } else {
                    println!("Desktop config applied successfully: {}", profile.name);
                }
            }
            Err(err) => {
                eprintln!("Warning: Failed to apply desktop config: {}", err);
            }
        }

        let _ = self.notifications_manager.notify_update(
            &title,
            &format!("{}\n{}\n{}", monitors_done, audio_done, desktop_done),
            notification_id,
            Some(3000),
            self.dry_run,
        );
    }
}
