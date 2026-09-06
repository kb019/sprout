#[derive(Clone, Debug)]
pub enum ResetEvent {
    Resetting,
    Reset,
    Failed(String),
}
