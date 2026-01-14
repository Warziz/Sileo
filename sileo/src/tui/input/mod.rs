use crossterm::event::KeyEvent;
use crate::app::state::{AppState, Screen};

pub mod welcome;
pub mod config;
pub mod chat;

pub fn handle(key: KeyEvent, state: &mut AppState) {
    match state.screen {
        Screen::Welcome => welcome::handle(key, state),
        Screen::Config => config::handle(key, state),
        Screen::Chat => chat::handle(key, state),
    }
}
