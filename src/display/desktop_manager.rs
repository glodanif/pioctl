use crate::display::desktop_error::DesktopError;
use crate::profile::desktop_config::DesktopConfig;

pub trait DesktopManager {
    fn dispatch_desktops(
        &self,
        desktop_config: &DesktopConfig,
        dry_run: bool,
    ) -> Result<(), DesktopError>;
}
