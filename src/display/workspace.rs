use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Workspace {
    pub index: String,
    pub monitor_name: String,
    pub active: bool,
    pub focused: bool,
}
