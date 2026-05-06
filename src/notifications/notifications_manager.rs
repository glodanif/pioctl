use crate::notifications::notification_error::NotificationError;
use std::process::Command;

const NOTIFY_CMD: &str = "notify-send";

pub struct NotificationsManager;

impl NotificationsManager {
    pub fn new() -> Self {
        NotificationsManager {}
    }

    pub fn notify(
        &self,
        title: &str,
        message: &str,
        dry_run: bool,
    ) -> Result<(), NotificationError> {
        self.notify_update(title, message, None, None, dry_run)
            .map(|_| ())
    }

    pub fn notify_update(
        &self,
        title: &str,
        body: &str,
        replace_id: Option<u32>,
        expire_ms: Option<u32>,
        dry_run: bool,
    ) -> Result<u32, NotificationError> {
        if dry_run {
            println!("[DRY RUN] Notification: {} | {}", title, body);
            //return Ok(0);
        }

        let mut args: Vec<String> = vec!["--print-id".to_string()];
        if let Some(id) = replace_id {
            args.push(format!("--replace-id={}", id));
        }
        if let Some(ms) = expire_ms {
            args.push(format!("--expire-time={}", ms));
        }
        args.push(title.to_string());
        args.push(body.to_string());

        let output = Command::new(NOTIFY_CMD)
            .args(&args)
            .output()
            .map_err(|_| {
                NotificationError::CommandExecutionError(format!(
                    "Failed to execute command {}",
                    NOTIFY_CMD
                ))
            })?;

        if output.status.success() {
            let id_str = String::from_utf8_lossy(&output.stdout)
                .trim()
                .to_string();
            id_str.parse::<u32>().map_err(|_| {
                NotificationError::CommandExecutionError(format!(
                    "Failed to parse notification ID: {}",
                    id_str
                ))
            })
        } else {
            Err(NotificationError::CommandExecutionError(format!(
                "Command {} failed for: {} | {}",
                NOTIFY_CMD, title, body
            )))
        }
    }
}
