#[derive(Clone, Debug)]
pub enum HeatmapDataEvent {
    Fetching(i32, i32),
    Fetched(i32, i32, Vec<(String, bool, i32)>),
    Failed(i32, i32, String),
}
