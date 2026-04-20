use std::collections::HashMap;

use chrono::{Datelike, Local};

use crate::{
    config::{AppSettings, Rule, Wallpaper},
    daemon::WallpaperState,
};

#[derive(Clone, Debug)]
pub struct Evaluator {}

impl Evaluator {
    pub fn new() -> Self {
        Self {}
    }

    /// Evaluates the background image file path for a given AppSettings struct.
    pub fn evaluate_wallpaper(&self, settings: &AppSettings) -> WallpaperState {
        let now = Local::now().naive_local();
        let mut wallpapers_by_monitor = HashMap::new();

        let active_wallpapers = settings.wallpapers.iter().filter(|wp| {
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

        for wallpaper in active_wallpapers {
            for monitor in &wallpaper.monitors {
                wallpapers_by_monitor.insert(monitor.clone(), wallpaper.clone());
            }
        }

        WallpaperState {
            wallpapers: wallpapers_by_monitor,
        }
    }
}
