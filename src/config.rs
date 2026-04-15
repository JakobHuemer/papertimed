use chrono::{DateTime, FixedOffset, Offset, TimeZone, WeekdaySet};

#[derive(Clone, Debug)]
pub struct Wallpaper {
    file: String,
}

#[derive(Clone, Debug)]
pub struct RepeatedSchedule {
    schedule: Schedule,
    repetition: Repetition,
}

#[derive(Clone, Copy, Debug)]
pub enum Repetition {
    Day,
    Week,
    Year,
}

#[derive(Clone, Debug)]
pub enum Schedule {
    Day {
        from: DateTime<FixedOffset>,
        to: DateTime<FixedOffset>,
    },
    Week(WeekdaySet),
    Year(Vec<u32>),
}
