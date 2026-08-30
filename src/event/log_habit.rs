use crate::model::habit::HabitLog;

#[derive(Clone, Debug)]
pub enum LogHabitEvent {
    Logging(i32),
    Logged(HabitLog),
    Failed(i32, String),
}
