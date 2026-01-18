pub mod state;
pub mod run;
pub mod action;

use std::sync::mpsc::{Sender,Receiver};

use state::AppState;

pub struct App {
    pub state: AppState,
}

impl App {
    pub fn new(
        tx_to_backend: Sender<String>, 
        rx_from_backend: Receiver<String>,
        tx_from_backend: Sender<String>,
        rx_to_backend: Receiver<String>
    ) -> Self {
        Self {
            state: AppState::new(tx_to_backend,rx_from_backend, tx_from_backend, rx_to_backend),

        }
    }
}