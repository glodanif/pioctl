use crate::display::desktop_error::DesktopError;
use crate::display::desktop_manager::DesktopManager;
use crate::profile::desktop_config::DesktopConfig;
use std::io::Error;
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, ExitStatus, Output};

const HYPRLAND_CMD: &str = "hyprctl";

pub struct HyprlandDesktopManager;

impl DesktopManager for HyprlandDesktopManager {
    fn dispatch_desktops(
        &self,
        desktop_config: &DesktopConfig,
        dry_run: bool,
    ) -> Result<(), DesktopError> {
        for workspace in &desktop_config.workspaces {
            match self.run(&["dispatch", "workspace", workspace.index.as_str()], dry_run) {
                Ok(output) if output.status.success() => {}
                Ok(output) => {
                    let msg = String::from_utf8_lossy(&output.stdout);
                    return Err(DesktopError::CommandExecutionError(format!(
                        "Failed to switch to workspace {}: {}",
                        workspace.index,
                        msg.trim()
                    )));
                }
                Err(e) => {
                    return Err(DesktopError::CommandExecutionError(format!(
                        "Failed to execute command for workspace {}: {}",
                        workspace.index, e
                    )));
                }
            }

            match self.run(
                &[
                    "dispatch",
                    "moveworkspacetomonitor",
                    workspace.index.as_str(),
                    workspace.monitor_name.as_str(),
                ],
                dry_run,
            ) {
                Ok(output) if output.status.success() => {}
                Ok(output) => {
                    let msg = String::from_utf8_lossy(&output.stdout);
                    return Err(DesktopError::CommandExecutionError(format!(
                        "Failed to move workspace {} to monitor {}: {}",
                        workspace.index,
                        workspace.monitor_name,
                        msg.trim()
                    )));
                }
                Err(e) => {
                    return Err(DesktopError::CommandExecutionError(format!(
                        "Failed to execute command to move workspace {} to monitor {}: {}",
                        workspace.index, workspace.monitor_name, e
                    )));
                }
            }
        }

        let mut active_workspaces: Vec<_> = desktop_config
            .workspaces
            .iter()
            .filter(|workspace| workspace.active)
            .collect();
        active_workspaces.sort_by_key(|workspace| workspace.focused);

        for workspace in &active_workspaces {
            match self.run(&["dispatch", "workspace", workspace.index.as_str()], dry_run) {
                Ok(output) if output.status.success() => {}
                Ok(output) => {
                    let msg = String::from_utf8_lossy(&output.stdout);
                    return Err(DesktopError::CommandExecutionError(format!(
                        "Failed to activate workspace {}: {}",
                        workspace.index,
                        msg.trim()
                    )));
                }
                Err(e) => {
                    return Err(DesktopError::CommandExecutionError(format!(
                        "Failed to execute command to activate workspace {}: {}",
                        workspace.index, e
                    )));
                }
            }
        }

        Ok(())
    }
}

impl HyprlandDesktopManager {
    fn run(&self, args: &[&str], dry_run: bool) -> Result<Output, Error> {
        if dry_run {
            println!("[DRY RUN] {} {}", HYPRLAND_CMD, args.join(" "));
            Ok(Output {
                status: ExitStatus::from_raw(0),
                stdout: Vec::new(),
                stderr: Vec::new(),
            })
        } else {
            Command::new(HYPRLAND_CMD).args(args).output()
        }
    }
}
