use std::{collections::HashMap, time::Duration};

use tokio::{sync::broadcast::Receiver, time::sleep};

use crate::{
    adapter::{
        AdapterDispatcher, WallpaperAdapter, custom::CustomAdapter, wpaperd::WpaperdAdapter,
    },
    config::{Adapter, AppSettings, Wallpaper},
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
    pub wallpapers: HashMap<String, Wallpaper>,
}

impl Daemon {
    pub fn new(settings: AppSettings, settings_rx: Receiver<AppSettings>) -> Self {
        let adapter = match settings.global.adapter.clone() {
            Adapter::Wpaperd => AdapterDispatcher::Wpaperd(WpaperdAdapter::default()),
            Adapter::Hyprpaper => AdapterDispatcher::Custom(CustomAdapter {
                command: "hyprctl hyprpaper wallpaper '{{ monitor }},{{ image }}'".to_string(),
            }),
            Adapter::Custom(command) => AdapterDispatcher::Custom(CustomAdapter { command }),
        };

        Self {
            evaluator: Evaluator::new(),
            state: WallpaperState::default(),
            settings_rx,
            settings,
            adapter,
        }
    }

    pub async fn start(&mut self) {
        loop {
            let wallpaper_state = self.evaluator.evaluate_wallpaper(&self.settings);

            self.state = wallpaper_state.clone();

            let ret = match &mut self.adapter {
                // AdapterDispatcher::Hyprpaper(a) => a.update(self.state.clone()).await,
                AdapterDispatcher::Wpaperd(a) => a.update(self.state.clone()).await,
                AdapterDispatcher::Custom(a) => a.update(self.state.clone()).await,
            };

            match ret {
                Err(e) => {
                    dbg!(&e);
                    panic!("{}", e)
                }
                _ => {}
            }

            sleep(Duration::from_secs(2)).await;
        }
    }
}
