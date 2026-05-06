use crate::display::display_error::DisplayError;
use crate::display::monitor::Monitor;
use crate::profile::monitors_config::MonitorsConfig;

pub trait DisplayManager {
    fn get_monitors(&self, dry_run: bool) -> Result<Vec<Monitor>, DisplayError>;
    fn get_monitors_json(&self, dry_run: bool) -> Result<String, DisplayError>;
    fn set_monitors_config(&self, config: &MonitorsConfig, dry_run: bool) -> Result<(), DisplayError>;
}
