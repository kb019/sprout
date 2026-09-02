use crate::model::habit::BestStreak;

#[derive(Clone, Debug)]
pub enum BestStreaksEvent {
    Fetching,
    Fetched(Vec<BestStreak>),
    Failed(String),
}
