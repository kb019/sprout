#[derive(Clone, Debug)]
pub enum DailyProgressEvent {
    Fetching(i32),
    Fetched(i32, i32),
    Failed(i32, String),
}

#[derive(Clone, Debug)]
pub enum WeeklyProgressEvent {
    Fetching(i32),
    Fetched(i32, i32),
    Failed(i32, String),
}

#[derive(Clone, Debug)]
pub enum MonthlyProgressEvent {
    Fetching(i32),
    Fetched(i32, i32),
    Failed(i32, String),
}

#[derive(Clone, Debug)]
pub enum YearlyProgressEvent {
    Fetching(i32),
    Fetched(i32, i32),
    Failed(i32, String),
}
