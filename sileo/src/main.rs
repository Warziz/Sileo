mod tui;
mod messaging;
mod app;

use std::sync::mpsc::channel;

use crate::app::App;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // make channel
    //tui to backend
    let (tx_to_backend, rx_to_backend) = channel::<String>();
    //backend to tui
    let (tx_from_backend, rx_from_backend) = channel::<String>();

    // create tui
    let mut app = App::new(tx_to_backend, rx_from_backend, tx_from_backend, rx_to_backend);

    color_eyre::install()?;
    let terminal = ratatui::init();
    app.run(terminal)?; // gère Welcome, Config, Chat

    Ok(())

}
