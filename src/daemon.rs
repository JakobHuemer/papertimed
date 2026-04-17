use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

use tokio::{process::Command, sync::broadcast::Receiver, time::sleep};

use crate::{
    adapter::{
        AdapterDispatcher, AdapterInput, WallpaperAdapter, hyprpaper::HyprpaperAdapter,
        wpaperd::WpaperdAdapter,
    },
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
            adapter: AdapterDispatcher::Hyprpaper(HyprpaperAdapter::default()),
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

                match &mut self.adapter {
                    AdapterDispatcher::Hyprpaper(a) => {
                        println!("LAJSDÖAJSDÖLJASDÖLJASÖLDJÖLASJDÖL");
                        if let Err(e) = a.update(self.state.clone()).await {
                            dbg!(e);
                        }
                    }
                    AdapterDispatcher::Wpaperd(a) => {
                        if let Err(e) = a.update(self.state.clone()).await {
                            dbg!(e);
                        }
                    }
                };

                dbg!(&self.state);
            } else {
                println!("Nothing");
            }

            sleep(Duration::from_secs(2)).await;
        }
    }
}
