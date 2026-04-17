use crate::{adapter::wpaperd::WpaperdAdapter, daemon::WallpaperState};

pub mod wpaperd;

pub trait WallpaperAdapter: Default {
    type Input;
    type Error: std::error::Error;

    async fn update(&mut self, input: Self::Input) -> Result<(), Self::Error>;
}

#[derive(Clone, Debug)]
pub enum AdapterDispatcher {
    Wpaperd(WpaperdAdapter),
}

#[derive(Debug, Clone)]
pub enum AdapterInput {
    Wpaperd(WallpaperState),
}

impl AdapterDispatcher {
    async fn update(&mut self, input: AdapterInput) -> Result<(), Box<dyn std::error::Error>> {
        match (self, input) {
            (Self::Wpaperd(a), AdapterInput::Wpaperd(b)) => a.update(b).await.map_err(Into::into),
        }
    }
}
