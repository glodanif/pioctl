use crate::cli::utils::run;
use crate::display::desktop_error::DesktopError;
use crate::display::desktop_manager::DesktopManager;
use crate::profile::desktop_config::DesktopConfig;
use std::process::Output;

const HYPRLAND_CMD: &str = "hyprctl";

pub struct HyprlandDesktopManager;

impl DesktopManager for HyprlandDesktopManager {
    fn dispatch_desktops(
        &self,
        desktop_config: &DesktopConfig,
        dry_run: bool,
    ) -> Result<(), DesktopError> {
        for workspace in &desktop_config.workspaces {
            self.activate_workspace(workspace.index.as_str(), dry_run)?;
            self.move_workspace_to_monitor(
                workspace.index.as_str(),
                workspace.monitor_name.as_str(),
                dry_run,
            )?;
        }

        let mut active_workspaces: Vec<_> = desktop_config
            .workspaces
            .iter()
            .filter(|workspace| workspace.active)
            .collect();
        active_workspaces.sort_by_key(|workspace| workspace.focused);

        for workspace in &active_workspaces {
            self.focus_monitor(workspace.monitor_name.as_str(), dry_run)?;
            self.activate_workspace(workspace.index.as_str(), dry_run)?;
        }

        Ok(())
    }
}

impl HyprlandDesktopManager {
    fn activate_workspace(&self, index: &str, dry_run: bool) -> Result<Output, DesktopError> {
        match run(HYPRLAND_CMD, &["dispatch", "workspace", index], dry_run) {
            Ok(output) if output.status.success() => Ok(output),
            Ok(output) => {
                let msg = String::from_utf8_lossy(&output.stdout);
                Err(DesktopError::CommandExecutionError(format!(
                    "Failed to activate workspace {}: {}",
                    index,
                    msg.trim()
                )))
            }
            Err(e) => Err(DesktopError::CommandExecutionError(format!(
                "Failed to execute command to activate workspace {}: {}",
                index, e
            ))),
        }
    }

    fn focus_monitor(&self, monitor_name: &str, dry_run: bool) -> Result<Output, DesktopError> {
        match run(HYPRLAND_CMD, &["dispatch", "focusmonitor", monitor_name], dry_run) {
            Ok(output) if output.status.success() => Ok(output),
            Ok(output) => {
                let msg = String::from_utf8_lossy(&output.stdout);
                Err(DesktopError::CommandExecutionError(format!(
                    "Failed to focus monitor {}: {}",
                    monitor_name,
                    msg.trim()
                )))
            }
            Err(e) => Err(DesktopError::CommandExecutionError(format!(
                "Failed to execute command to focus monitor {}: {}",
                monitor_name, e
            ))),
        }
    }

    fn move_workspace_to_monitor(
        &self,
        index: &str,
        monitor_name: &str,
        dry_run: bool,
    ) -> Result<Output, DesktopError> {
        match run(
            HYPRLAND_CMD,
            &["dispatch", "moveworkspacetomonitor", index, monitor_name],
            dry_run,
        ) {
            Ok(output) if output.status.success() => Ok(output),
            Ok(output) => {
                let msg = String::from_utf8_lossy(&output.stdout);
                return Err(DesktopError::CommandExecutionError(format!(
                    "Failed to move workspace {} to monitor {}: {}",
                    index,
                    monitor_name,
                    msg.trim()
                )));
            }
            Err(e) => {
                return Err(DesktopError::CommandExecutionError(format!(
                    "Failed to execute command to move workspace {} to monitor {}: {}",
                    index, monitor_name, e
                )));
            }
        }
    }
}
