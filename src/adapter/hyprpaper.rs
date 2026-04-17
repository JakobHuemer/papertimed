use std::{fs, path::PathBuf};

use thiserror::Error;
use tokio::process::Command;

use crate::{adapter::WallpaperAdapter, daemon::WallpaperState};

#[derive(Debug, Default, Clone)]
pub struct HyprpaperAdapter {}

#[derive(Debug, Clone, Error)]
pub enum HyprpaperError {
    #[error("Could not create all directories: {0}")]
    CreateAllDirs(PathBuf),
    #[error("Could not write to config at {0}")]
    WriteToConfig(PathBuf),
    #[error("hyprpaper is not installed")]
    HyprpaperNotInstalled,
    #[error("No wallpaper is currently set")]
    NoWallpaper,
    #[error("Hyprpaper command failed wit exit code {0} and message: {1}")]
    HyprpaperFailedWith(i32, String),
}

impl WallpaperAdapter for HyprpaperAdapter {
    type Error = HyprpaperError;
    type Input = WallpaperState;

    async fn update(&mut self, input: Self::Input) -> Result<(), Self::Error> {
        if let Some(bg) = input.current_wallpaper {
            // hyprctl hyprpaper wallpaper '[mon], [path], [fit_mode]'

            let output = Command::new("hyprctl")
                .args(&[
                    "hyprpaper",
                    "wallpaper",
                    format!(",{}", bg.filename).as_str(),
                ])
                .output()
                .await
                .map_err(|_e| HyprpaperError::HyprpaperNotInstalled)?;

            if !output.status.success() {
                return Err(HyprpaperError::HyprpaperFailedWith(
                    output.status.code().unwrap_or_default(),
                    String::from_utf8(output.stdout).unwrap(),
                ));
            }
        }

        Err(HyprpaperError::NoWallpaper)
    }
}
