use thiserror::Error;

use crate::{
    adapter::{
        hyprpaper::{HyprpaperAdapter, HyprpaperError},
        wpaperd::{WpaperdAdapter, WpaperdError},
    },
    daemon::WallpaperState,
};

pub mod hyprpaper;
pub mod wpaperd;

pub trait WallpaperAdapter: Default {
    type Input;

    async fn update(&mut self, input: Self::Input) -> Result<(), AdapterError>;
}

#[derive(Clone, Debug)]
pub enum AdapterDispatcher {
    Wpaperd(WpaperdAdapter),
    Hyprpaper(HyprpaperAdapter),
}

#[derive(Debug, Clone)]
pub enum AdapterInput {
    Wpaperd(WallpaperState),
}

#[derive(Clone, Debug, Error)]
pub enum AdapterError {
    #[error("hyprpaper: {0}")]
    Hyprpaper(#[from] HyprpaperError),

    #[error("wpaperd: {0}")]
    Wpaperd(#[from] WpaperdError),

    #[error("background utility '{0}' not installed")]
    UtilityNotInstalled(String),

    #[error("{utility} failed with ({status_code}): {error_out}")]
    UtilityFailedWith {
        utility: String,
        status_code: i32,
        error_out: String,
    },
}
