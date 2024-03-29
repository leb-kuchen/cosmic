#[rustfmt::skip]
mod backlight;
mod app;
mod config;
mod localize;
mod power_daemon;
mod upower;

mod upower_device;
mod upower_kbdbacklight;
use config::APP_ID;
use log::info;

use localize::localize;

use crate::config::{PROFILE, VERSION};

fn main() -> cosmic::iced::Result {
    // Initialize logger
    pretty_env_logger::init();
    info!("Iced Workspaces Applet ({})", APP_ID);
    info!("Version: {} ({})", VERSION, PROFILE);

    // Prepare i18n
    localize();

    app::run()
}
