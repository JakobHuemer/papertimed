use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

use tokio::{process::Command, sync::broadcast::Receiver, time::sleep};

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

    pub async fn start(&mut self) {
        loop {
            if let Some(bg) = self.evaluator.evaluate_wallpaper(&self.settings)
                && !self
                    .previous_image
                    .as_ref()
                    .is_some_and(|prev_bg| prev_bg.filename == bg.filename)
            {
                println!("BG: {:?}", bg);

                self.previous_image = Some(bg.clone());

                // change wallpaper
                let config_path = PathBuf::from("./out/config/wpaperd/wallpaper.toml");

                let new_content = format!("[default]\npath = '{}'", bg.filename);

                fs::create_dir_all(&config_path.parent().unwrap()).unwrap();
                fs::write(config_path, new_content).unwrap();

                let reload_command = Command::new("wpwaperctl")
                    .args(&["reload-wallpaper"])
                    .status();
            } else {
                println!("Nothing");
            }

            sleep(Duration::from_secs(2)).await;
        }
    }
}
