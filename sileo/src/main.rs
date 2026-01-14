mod tui;
mod messaging;
mod app;

use std::sync::mpsc::channel;
use crate::app::App;
use crate::messaging::client::MessagingClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --- 1. Lancer le TUI ---
    let mut app = App::new();

    // Le TUI va remplir cette config via l'écran Config
    let config = app.run_until_config()?; // tu l’as déjà conceptuellement

    // --- 2. Channels ---
    let (tx_to_backend, rx_to_backend) = channel::<String>();
    let (tx_from_backend, rx_from_backend) = channel::<String>();

    // --- 3. Injecter dans le state ---
    app.state.tx_backend = tx_to_backend;
    app.state.rx_backend = rx_from_backend;

    // --- 4. Lancer le backend (TES fonctions existantes) ---
    let client = MessagingClient::new(config, incoming, outgoing)?;
    client.start();

    // --- 5. Lancer la boucle principale TUI ---
    app.run_chat_loop()?;

    Ok(())
}
