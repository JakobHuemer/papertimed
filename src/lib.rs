mod config;
mod daemon;

pub fn run() {
    let d = daemon::Daemon::new();

    d.start();
}
