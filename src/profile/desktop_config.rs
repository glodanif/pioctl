use serde::{Deserialize, Serialize};
use crate::display::workspace::Workspace;

#[derive(Serialize, Deserialize)]
pub struct DesktopConfig {
    pub delay_before_ms: Option<u32>,
    pub workspaces: Vec<Workspace>,
}
