mod tui;
mod messaging;
mod app;

use std::sync::mpsc::channel;

use crate::app::App;
use crate::messaging::client::MessagingClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // 1. Créer les channels
    let (tx_to_backend, rx_to_backend) = channel::<String>();
    let (tx_from_backend, rx_from_backend) = channel::<String>();

    // 2. Lancer le TUI
    let mut app = App::new(tx_to_backend, rx_from_backend);

    color_eyre::install()?;
    let terminal = ratatui::init();
    app.run(terminal)?; // gère Welcome, Config, Chat

    // 4. Quand l'utilisateur a validé la config
    if let Some(config) = app.state.config.take() {
        let mut client = MessagingClient::new(
            config,
            tx_from_backend.clone(),
            rx_to_backend,
        )?;

        client.start();

    }

    Ok(())

}
