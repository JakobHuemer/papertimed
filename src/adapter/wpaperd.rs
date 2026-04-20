use std::{env, fs, path::PathBuf};

use thiserror::Error;
use tokio::process::Command;

const WPAPERD_CONFIG_LOCATION: &'static str = ".config/wpaperd/wallpaper.toml";

use crate::{
    adapter::{AdapterError, WallpaperAdapter, write_file_save},
    daemon::WallpaperState,
};

#[derive(Clone, Debug, Copy, Default)]
pub struct WpaperdAdapter {}

#[derive(Debug, Clone, Error)]
pub enum WpaperdError {}

impl WallpaperAdapter for WpaperdAdapter {
    type Input = WallpaperState;

    async fn update(&mut self, input: Self::Input) -> Result<(), AdapterError> {
        let mut config_str = String::new();

        for (monitor, wallpaper) in input.wallpapers {
            config_str
                .push_str(format!("[{}]\npath = \"{}\"\n\n", monitor, wallpaper.filename).as_str());
        }

        let config_path = env::home_dir().unwrap();
        let config_path = config_path.join(PathBuf::from(WPAPERD_CONFIG_LOCATION));

        write_file_save(&config_path, config_str)?;

        Command::new("wpaperctl")
            .args(&["reload-wallpaper"])
            .status()
            .await
            .map_err(|_e| AdapterError::UtilityNotInstalled("wpaperctl".to_string()))?;

        Ok(())
    }
}
