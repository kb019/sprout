#[derive(Clone, Debug)]
pub enum HeatmapYearHabitsEvent {
    Fetching,
    Fetched(Vec<(i32, Vec<i32>)>),
    Failed(String),
}
