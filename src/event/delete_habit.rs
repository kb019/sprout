#[derive(Clone, Debug)]
pub enum DeleteHabitEvent {
    Deleting(i32),
    Deleted(i32),
    Failed(i32, String),
}
