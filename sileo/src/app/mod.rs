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
        rx_from_backend: Receiver<BackendEvent>,
        tx_from_backend: Sender<BackendEvent>,
    ) -> Self {
        Self {
            state: AppState::new(rx_from_backend, tx_from_backend),

        }
    }
}