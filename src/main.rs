mod gui;
mod permissions;
mod database;
mod utils;
mod worker;

use log::info;
use anyhow::Result;

fn main() -> Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp_millis()
        .init();

    info!("Rusty Permission Changer started");

    // Initialize database
    if let Err(e) = database::init_database() {
        log::error!("Failed to initialize database: {}", e);
        return Err(e);
    }

    // Create and run GUI
    gui::run_app()?;

    info!("Rusty Permission Changer closed");
    Ok(())
}
