// RMD - A fast, native Windows Markdown editor
// Built with Rust and egui

mod app;
mod config;
mod editor;
mod markdown;
mod preview;
mod theme;
mod ui;
mod utils;

use eframe::NativeOptions;

fn main() -> eframe::Result {
    // Initialize logging
    env_logger::init();

    // Load configuration
    let config = config::Config::load_or_default();

    // Create native options with custom window settings
    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("RMD - Markdown Editor"),
        ..Default::default()
    };

    // Run the application
    eframe::run_native(
        "RMD",
        native_options,
        Box::new(|cc| Ok(Box::new(app::RmdApp::new(cc, config)))),
    )
}
