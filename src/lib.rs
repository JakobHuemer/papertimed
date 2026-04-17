use crate::config::AppConfig;

mod config;
mod daemon;
mod evaluator;

pub async fn run() {
    let app_config = AppConfig::new();

    let d = daemon::Daemon::new(
        app_config.app_settings.clone(),
        app_config.config_rx.resubscribe(),
    );

    dbg!(app_config);

    d.start().await;
}
