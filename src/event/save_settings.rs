#[derive(Clone, Debug)]
pub enum SettingsSaveEvent {
    Saving,
    Saved,
    Failed(String),
}
