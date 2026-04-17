use std::time::Duration;

use tokio::{sync::broadcast::Receiver, time::sleep};

use crate::{
    config::{AppSettings, Wallpaper},
    evaluator::Evaluator,
};

pub struct Daemon {
    evaluator: Evaluator,
    settings: AppSettings,
    settings_rx: Receiver<AppSettings>,
    previous_image: Option<Wallpaper>,
}

impl Daemon {
    pub fn new(settings: AppSettings, settings_rx: Receiver<AppSettings>) -> Self {
        Self {
            evaluator: Evaluator::new(),
            settings_rx,
            settings,
            previous_image: None,
        }
    }

    pub async fn start(&self) {
        loop {
            if let Some(bg) = self.evaluator.evaluate_wallpaper(&self.settings) {
                println!("BG: {:?}", bg);
            }

            sleep(Duration::from_secs(2)).await;
        }
    }
}
