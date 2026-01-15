pub mod state;
pub mod run;
pub mod action;

use std::sync::mpsc::{Sender,Receiver};

use state::AppState;

pub struct App {
    pub state: AppState,
    tx_backend: Sender<String>,
    rx_backend: Receiver<String>,
}

impl App {
    pub fn new(tx_backend: Sender<String>, rx_backend: Receiver<String>) -> Self {
        Self {
            state: AppState::new(),
            tx_backend,
            rx_backend,
        }
    }
}