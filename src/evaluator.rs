use chrono::{Datelike, Local};

use crate::config::{AppSettings, Rule, Wallpaper};

#[derive(Clone, Debug)]
pub struct Evaluator {}

impl Evaluator {
    pub fn new() -> Self {
        Self {}
    }

    /// Evaluates the background image file path for a given AppSettings struct.
    pub fn evaluate_wallpaper<'b>(&self, settings: &'b AppSettings) -> Option<&'b Wallpaper> {
        // walk through
        let now = Local::now().naive_local();

        let mut bg = settings.wallpapers.iter().filter(|wp| {
            wp.schedules.iter().any(|schedule| {
                schedule.rules.iter().all(|rule| match rule {
                    Rule::DayTime { from, to } => *from <= now.time() && *to > now.time(),
                    Rule::WeekDays(wrapped_week_day_set) => {
                        wrapped_week_day_set.week_day.contains(now.weekday())
                    }
                    Rule::YearDays(items) => items.contains(&now.ordinal()),
                })
            })
        });

        bg.next()
    }
}
