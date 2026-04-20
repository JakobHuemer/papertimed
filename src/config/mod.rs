use std::env;

use config::Config;
use thiserror::Error;
use tokio::sync::broadcast::{self, Receiver, Sender};

use crate::config::raw::RawAppSettings;
pub use crate::config::raw::{Adapter, GlobalSettings, Rule, Schedule};

mod raw;

const CONFIGURATION_LOCATION: &str = ".config/papertimed/config";

#[derive(Debug)]
pub struct AppConfig {
    pub config_rx: Receiver<AppSettings>,
    config_tx: Sender<AppSettings>,

    pub app_settings: AppSettings,
}

impl AppConfig {
    pub fn new() -> Self {
        let settings = Self::load_config();

        dbg!(&settings);

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
        let config_path = env::var("PAPERTIMED_CONFIG_PATH").unwrap_or_else(|_| {
            format!(
                "{}/{}",
                env::var("HOME").expect("HOME not set"),
                CONFIGURATION_LOCATION
            )
        });

        let settings = Config::builder()
            .add_source(config::File::with_name(config_path.as_str()))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .unwrap();

        let app_settings = settings.try_deserialize::<RawAppSettings>().unwrap();

        AppSettings::try_from(app_settings).unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct Wallpaper {
    pub filename: String,
    pub monitors: Vec<String>,
    pub schedules: Vec<Schedule>,
}

#[derive(Debug, Clone)]
pub struct AppSettings {
    pub wallpapers: Vec<Wallpaper>,
    pub global: GlobalSettings,
}

#[derive(Debug, Clone, Error)]
pub enum AppSettingsParseError {
    #[error("schedule '{0}' does not exist")]
    ScheduleDoesNotExist(String),
}

impl TryFrom<RawAppSettings> for AppSettings {
    type Error = AppSettingsParseError;

    fn try_from(app_settings: RawAppSettings) -> Result<Self, Self::Error> {
        let mut wallpapers: Vec<Wallpaper> = vec![];

        let mut schedules_map = std::collections::HashMap::new();
        for schedule in app_settings.schedules {
            schedules_map.insert(schedule.id.clone(), schedule);
        }

        for wp in app_settings.wallpapers {
            let schedules: Vec<Schedule> = wp
                .schedules
                .iter()
                .map(|sched_id| {
                    schedules_map.get(sched_id).cloned().ok_or_else(|| {
                        AppSettingsParseError::ScheduleDoesNotExist(sched_id.clone())
                    })
                })
                .collect::<Result<Vec<Schedule>, AppSettingsParseError>>()?;

            let new_wp = Wallpaper {
                schedules,
                filename: wp.filename,
                monitors: wp.monitors,
            };

            wallpapers.push(new_wp);
        }

        Ok(Self {
            wallpapers,
            global: app_settings.global,
        })
    }
}
