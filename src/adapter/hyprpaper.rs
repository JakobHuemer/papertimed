use std::path::PathBuf;

use thiserror::Error;
use tokio::process::Command;

use crate::{
    adapter::{AdapterError, WallpaperAdapter},
    config::Adapter,
    daemon::WallpaperState,
};

#[derive(Debug, Default, Clone)]
pub struct HyprpaperAdapter {}

#[derive(Debug, Clone, Error)]
pub enum HyprpaperError {
    #[error("Could not write to config at {0}")]
    WriteToConfig(PathBuf),
    #[error("No wallpaper is currently set")]
    NoWallpaper,
    #[error("Hyprpaper command failed wit exit code {0} and message: {1}")]
    HyprpaperFailedWith(i32, String),
}

impl WallpaperAdapter for HyprpaperAdapter {
    type Input = WallpaperState;
    async fn update(&mut self, input: Self::Input) -> Result<(), AdapterError> {
        if input.wallpapers.is_empty() {
            return Ok(());
        }

        for (monitor, wallpaper) in &input.wallpapers {
            // hyprctl hyprpaper wallpaper '[mon], [path]'
            let output = Command::new("hyprctl")
                .args(&[
                    "hyprpaper",
                    "wallpaper",
                    format!("{},{}", monitor, wallpaper.filename).as_str(),
                ])
                .output()
                .await
                .map_err(|_e| AdapterError::UtilityNotInstalled("hyprpaper".to_string()))?;

            if !output.status.success() {
                return Err(AdapterError::UtilityFailedWith {
                    status_code: output.status.code().unwrap_or_default(),
                    error_out: String::from_utf8(output.stdout).unwrap(),
                    utility: "hyprpaper".to_string(),
                });
            }
        }

        Ok(())
    }
}
