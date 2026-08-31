#[derive(Clone, Debug)]
pub enum GetStreakEvent {
    Fetching(i32),
    Fetched(i32, i32), // habit_id, streak
    Failed(i32, String),
}
