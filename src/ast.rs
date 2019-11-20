#[derive(Debug)]
pub struct HotkeyOverride {
    pub ability_id: String,
    pub overrides: Vec<(String, String)>,
}
