#[derive(Default)]
pub struct SettingsSaveState {
    is_saving: bool,
}

impl SettingsSaveState {
    pub fn new() -> Self {
        Self { is_saving: false }
    }

    pub fn start_saving(&mut self) {
        self.is_saving = true;
    }

    pub fn stop_saving(&mut self) {
        self.is_saving = false;
    }

    pub fn is_saving(&self) -> bool {
        self.is_saving
    }
}
