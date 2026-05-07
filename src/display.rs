mod desktop_error;
pub mod desktop_manager;
pub mod display_error;
pub mod display_manager;
pub mod hyprland_desktop_manager;
pub mod hyprland_display_manager;
pub mod mode;
pub mod monitor;
pub mod size;
pub mod transformation;
pub mod workspace;

use desktop_manager::DesktopManager;
use display_manager::DisplayManager;
use hyprland_desktop_manager::HyprlandDesktopManager;
use hyprland_display_manager::HyprlandDisplayManager;

pub fn get_display_manager() -> Box<dyn DisplayManager> {
    Box::new(HyprlandDisplayManager)
}

pub fn get_desktop_manager() -> Box<dyn DesktopManager> {
    Box::new(HyprlandDesktopManager)
}
