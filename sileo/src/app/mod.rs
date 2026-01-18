pub mod state;
pub mod run;
pub mod action;

use std::sync::mpsc::{Sender,Receiver};

use state::AppState;

use crate::messaging::utils::event::BackendEvent;

pub struct App {
    pub state: AppState,
}

impl App {
    pub fn new(
        tx_to_backend: Sender<String>, 
        rx_from_backend: Receiver<BackendEvent>,
        tx_from_backend: Sender<BackendEvent>,
        rx_to_backend: Receiver<String>
    ) -> Self {
        Self {
            state: AppState::new(tx_to_backend,rx_from_backend, tx_from_backend, rx_to_backend),

        }
    }
}