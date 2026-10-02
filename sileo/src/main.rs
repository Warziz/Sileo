mod tui;
mod messaging;
mod app;

use std::sync::mpsc::channel;

use crate::{app::App, messaging::utils::event::BackendEvent};


/// Application entry point.
///
/// This function initializes communication channels, sets up the
/// terminal user interface, and starts the main application loop.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create communication channels
    // Backend -> TUI
    let (tx_from_backend, rx_from_backend) = channel::<BackendEvent>();

    // Initialize application state
    let mut app = App::new(
        rx_from_backend,
        tx_from_backend
    );

    // Install enhanced error reporting
    color_eyre::install()?;

    // Initialize terminal and start the TUI
    let terminal = ratatui::init();
    app.run(terminal)?;

    Ok(())
}