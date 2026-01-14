pub mod state;
pub mod run;
pub mod action;

use state::AppState;

pub struct App {
    pub state: AppState,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::new(),
        }
    }
}