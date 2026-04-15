use crate::config::AppConfig;

mod config;
mod daemon;
mod evaluator;

pub fn run() {
    let d = daemon::Daemon::new();

    let app_config = AppConfig::new();

    dbg!(app_config);

    d.start();
}
