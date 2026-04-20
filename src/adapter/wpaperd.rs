use std::{env, fs, path::PathBuf};

use thiserror::Error;
use tokio::process::Command;

use crate::{
    adapter::{AdapterError, WallpaperAdapter},
    daemon::WallpaperState,
};

#[derive(Clone, Debug, Copy, Default)]
pub struct WpaperdAdapter {}

#[derive(Debug, Clone, Error)]
pub enum WpaperdError {
    #[error("Could not create all directories: {0}")]
    CreateAllDirs(PathBuf),
    #[error("Could not write to config at {0}")]
    WriteToConfig(PathBuf),
}

impl WallpaperAdapter for WpaperdAdapter {
    type Input = WallpaperState;

    async fn update(&mut self, input: Self::Input) -> Result<(), AdapterError> {
        let mut config_str = String::new();

        for (monitor, wallpaper) in input.wallpapers {
            config_str
                .push_str(format!("[{}]\npath = \"{}\"\n\n", monitor, wallpaper.filename).as_str());
        }

        let config_path = env::home_dir().unwrap();
        let config_path = config_path.join(PathBuf::from(".config/wpaperd/wallpaper.toml"));

        let config_parent_dir = config_path.parent().to_owned().unwrap().to_path_buf();
        println!("{config_str}");

        fs::create_dir_all(&config_parent_dir)
            .map_err(|_| WpaperdError::CreateAllDirs(config_parent_dir))?;
        fs::write(&config_path, config_str)
            .map_err(|_| WpaperdError::WriteToConfig(config_path))?;

        Ok(())
    }
}
