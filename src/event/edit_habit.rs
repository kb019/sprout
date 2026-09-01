use crate::model::habit::Habit;

#[derive(Clone, Debug)]
pub enum EditHabitEvent {
    Editing(i32),
    Edited(Habit),
    Failed(i32, String),
}
