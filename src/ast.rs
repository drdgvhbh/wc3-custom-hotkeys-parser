use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HotkeyOverride {
    pub ability_id: String,
    pub overrides: Vec<(String, String)>,
}
