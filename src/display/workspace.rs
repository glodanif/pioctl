use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Workspace {
    pub index: String,
    pub monitor_name: String,
    #[serde(default)]
    pub active: bool,
    #[serde(default)]
    pub focused: bool,
}
