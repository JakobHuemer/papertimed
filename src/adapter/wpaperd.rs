use std::{fs, path::PathBuf, process::ExitStatus};

use thiserror::Error;
use tokio::process::Command;

use crate::{adapter::WallpaperAdapter, daemon::WallpaperState};

#[derive(Clone, Debug, Copy, Default)]
pub struct WpaperdAdapter {}

#[derive(Debug, Clone, Error)]
pub enum WpaperdError {
    #[error("Could not create all directories: {0}")]
    CreateAllDirs(PathBuf),
    #[error("Could not write to config at {0}")]
    WriteToConfig(PathBuf),
    #[error("wpaperd is not installed")]
    WpaperdNotInstalled,
    #[error("No wallpaper is currently set")]
    NoWallpaper,
}

impl WallpaperAdapter for WpaperdAdapter {
    type Input = WallpaperState;
    type Error = WpaperdError;

    async fn update(&mut self, input: Self::Input) -> Result<(), Self::Error> {
        if let Some(bg) = input.current_wallpaper {
            let config_path = PathBuf::from("./out/config/wpaperd/wallpaper.toml");

            let new_content = format!("[default]\npath = '{}'", bg.filename);

            fs::create_dir_all(&config_path.parent().unwrap()).unwrap();
            fs::write(config_path, new_content).unwrap();

            let status = Command::new("wpaperctl")
                .args(&["reload-wallpaper"])
                .status()
                .await
                .map_err(|e| WpaperdError::WpaperdNotInstalled)?;

            return Ok(());
        }

        Err(WpaperdError::NoWallpaper)
    }
}
