use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

use tokio::{process::Command, sync::broadcast::Receiver, time::sleep};

use crate::{
    adapter::{AdapterDispatcher, AdapterInput, WallpaperAdapter, wpaperd::WpaperdAdapter},
    config::{AppSettings, Wallpaper},
    evaluator::Evaluator,
};

#[derive(Debug)]
pub struct Daemon {
    evaluator: Evaluator,
    settings: AppSettings,
    settings_rx: Receiver<AppSettings>,
    state: WallpaperState,
    adapter: AdapterDispatcher,
}

#[derive(Default, Clone, Debug)]
pub struct WallpaperState {
    pub current_wallpaper: Option<Wallpaper>,
}

impl Daemon {
    pub fn new(settings: AppSettings, settings_rx: Receiver<AppSettings>) -> Self {
        Self {
            evaluator: Evaluator::new(),
            settings_rx,
            settings,
            state: WallpaperState::default(),
            adapter: AdapterDispatcher::Wpaperd(WpaperdAdapter::default()),
        }
    }

    pub async fn start(&mut self) {
        loop {
            if let Some(bg) = self.evaluator.evaluate_wallpaper(&self.settings)
                && !self
                    .state
                    .current_wallpaper
                    .as_ref()
                    .is_some_and(|prev_bg| prev_bg.filename == bg.filename)
            {
                println!("BG: {:?}", bg);

                self.state.current_wallpaper = Some(bg.clone());

                dbg!(&self.state);

                let _ = match &mut self.adapter {
                    AdapterDispatcher::Wpaperd(adapter) => adapter.update(self.state.clone()).await,
                }
                .inspect_err(|e| {
                    dbg!(e);
                });
            } else {
                println!("Nothing");
            }

            sleep(Duration::from_secs(2)).await;
        }
    }
}
