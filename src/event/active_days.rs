#[derive(Clone, Debug)]
pub enum ActiveDaysEvent {
    Fetching,
    Fetched(u32),
    Failed(String),
}
