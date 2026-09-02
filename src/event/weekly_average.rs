#[derive(Clone, Debug)]
pub enum WeeklyAverageEvent {
    Fetching,
    Fetched([u32; 7]),
    Failed(String),
}
