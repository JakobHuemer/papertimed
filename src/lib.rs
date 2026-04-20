use crate::config::AppConfig;

mod adapter;
mod config;
mod daemon;
mod evaluator;

pub async fn run() {
    let app_config = AppConfig::new();

    let mut d = daemon::Daemon::new(
        app_config.app_settings.clone(),
        app_config.config_rx.resubscribe(),
    );

    d.start().await;
}
