use chrono::{NaiveTime, WeekdaySet};
use config::Config;
use serde::Deserialize;
use tokio::sync::broadcast::{self, Receiver, Sender};

#[derive(Debug)]
pub struct AppConfig {
    pub config_rx: Receiver<AppSettings>,
    config_tx: Sender<AppSettings>,

    pub app_settings: AppSettings,
}

impl AppConfig {
    pub fn new() -> Self {
        let settings = Self::load_config();

        let (config_tx, config_rx) = broadcast::channel::<AppSettings>(16);

        Self {
            config_rx,
            config_tx,
            app_settings: settings,
        }
    }

    pub async fn reload(&mut self) {
        let settings = Self::load_config();

        self.app_settings = settings.clone();

        let _ = self.config_tx.send(settings);
    }

    fn load_config() -> AppSettings {
        let settings = Config::builder()
            .add_source(config::File::with_name("examples/config"))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .unwrap();

        settings.try_deserialize::<AppSettings>().unwrap()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AppSettings {
    pub global: GlobalSettings,
    #[serde(default)]
    pub wallpapers: Vec<Wallpaper>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GlobalSettings {
    pub adapter: Adapter,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Adapter {
    Wpaperd,
    Hyprpaper
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Wallpaper {
    pub filename: String,
    pub schedules: Vec<RepetitionSchedule>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RepetitionSchedule {
    #[serde(default)]
    pub rules: Vec<Rule>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Rule {
    Day { from: NaiveTime, to: NaiveTime },
    Week(WrappedWeekDaySet),
    Year(Vec<u32>),
}

#[derive(Debug, Clone)]
pub struct WrappedWeekDaySet {
    pub week_day: WeekdaySet,
}

impl<'de> Deserialize<'de> for WrappedWeekDaySet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct WeekdaySetVisitor;

        impl<'de> serde::de::Visitor<'de> for WeekdaySetVisitor {
            type Value = WrappedWeekDaySet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an array of unique weekdays")
            }

            fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
            where
                S: serde::de::SeqAccess<'de>,
            {
                use chrono::Weekday;

                let mut week_day = WeekdaySet::EMPTY;

                while let Some(day_str) = seq.next_element::<String>()? {
                    let day = match day_str.to_lowercase().as_str() {
                        "mon" | "monday" => Weekday::Mon,
                        "tue" | "tuesday" => Weekday::Tue,
                        "wed" | "wednesday" => Weekday::Wed,
                        "thu" | "thursday" => Weekday::Thu,
                        "fri" | "friday" => Weekday::Fri,
                        "sat" | "saturday" => Weekday::Sat,
                        "sun" | "sunday" => Weekday::Sun,
                        _ => {
                            return Err(serde::de::Error::custom(format!(
                                "invalid weekday: {}",
                                day_str
                            )));
                        }
                    };
                    week_day.insert(day);
                }

                Ok(WrappedWeekDaySet { week_day })
            }
        }

        deserializer.deserialize_seq(WeekdaySetVisitor)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;

    #[test]
    fn test_wrapped_weekday_set_two_unique_days() {
        let json = r#"["mon", "tue"]"#;
        let result: Result<WrappedWeekDaySet, _> = serde_json::from_str(json);
        assert!(result.is_ok());
        let wrapped = result.unwrap();
        assert_eq!(wrapped.week_day.len(), 2);
        assert!(wrapped.week_day.contains(chrono::Weekday::Mon));
        assert!(wrapped.week_day.contains(chrono::Weekday::Tue));
    }

    #[test]
    fn test_wrapped_weekday_set_three_unique_plus_duplicate() {
        let json = r#"["mon", "tue", "wed", "tue"]"#;
        let result: Result<WrappedWeekDaySet, _> = serde_json::from_str(json);
        assert!(result.is_ok());
        let wrapped = result.unwrap();
        assert_eq!(wrapped.week_day.len(), 3);
        assert!(wrapped.week_day.contains(chrono::Weekday::Mon));
        assert!(wrapped.week_day.contains(chrono::Weekday::Tue));
        assert!(wrapped.week_day.contains(chrono::Weekday::Wed));
    }

    #[test]
    fn test_wrapped_weekday_set_empty() {
        let json = r#"[]"#;
        let result: Result<WrappedWeekDaySet, _> = serde_json::from_str(json);
        assert!(result.is_ok());
        let wrapped = result.unwrap();
        assert_eq!(wrapped.week_day.len(), 0);
    }

    #[test]
    fn test_wrapped_weekday_set_all_days() {
        let json = r#"["mon", "tue", "wed", "thu", "fri", "sat", "sun"]"#;
        let result: Result<WrappedWeekDaySet, _> = serde_json::from_str(json);
        assert!(result.is_ok());
        let wrapped = result.unwrap();
        assert_eq!(wrapped.week_day.len(), 7);
        assert!(wrapped.week_day.contains(chrono::Weekday::Mon));
        assert!(wrapped.week_day.contains(chrono::Weekday::Tue));
        assert!(wrapped.week_day.contains(chrono::Weekday::Wed));
        assert!(wrapped.week_day.contains(chrono::Weekday::Thu));
        assert!(wrapped.week_day.contains(chrono::Weekday::Fri));
        assert!(wrapped.week_day.contains(chrono::Weekday::Sat));
        assert!(wrapped.week_day.contains(chrono::Weekday::Sun));
    }
}
