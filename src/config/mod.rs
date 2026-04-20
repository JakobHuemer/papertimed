use config::Config;
use thiserror::Error;
use tokio::sync::broadcast::{self, Receiver, Sender};

pub use crate::config::raw::{Adapter, GlobalSettings, Rule, Schedule, WrappedWeekDaySet};
use crate::config::raw::{RawAppSettings, RawWallpaper};

mod raw;

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

        todo!()
    }

    fn load_config() -> AppSettings {
        let settings = Config::builder()
            .add_source(config::File::with_name("examples/config"))
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
pub enum AppSettingsParseError {}

impl TryFrom<RawAppSettings> for AppSettings {
    type Error = AppSettingsParseError;

    fn try_from(app_settings: RawAppSettings) -> Result<Self, Self::Error> {
        let mut wallpapers: Vec<Wallpaper> = vec![];

        for wp in app_settings.wallpapers {
            println!("------------------------------------\n{}", wp.filename);
            let schedules: Vec<Schedule> = app_settings
                .schedules
                .iter()
                .filter(|schedule| wp.schedules.contains(&schedule.id))
                .map(|s| s.clone())
                .collect();

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
