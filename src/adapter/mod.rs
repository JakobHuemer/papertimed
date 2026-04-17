use crate::{
    adapter::{hyprpaper::HyprpaperAdapter, wpaperd::WpaperdAdapter},
    daemon::WallpaperState,
};

pub mod hyprpaper;
pub mod wpaperd;

pub trait WallpaperAdapter: Default {
    type Input;
    type Error: std::error::Error;

    async fn update(&mut self, input: Self::Input) -> Result<(), Self::Error>;
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
