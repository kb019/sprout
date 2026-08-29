use crate::model::habit::Habit;

#[derive(Clone, Debug)]
pub enum AddHabitEvent {
    Adding,
    Added(Habit),
    Failed(String),
}
