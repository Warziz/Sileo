use ratatui::Frame;
use crate::app::state::{AppState, Screen};

pub mod welcome;
pub mod config;
pub mod chat;

pub fn render(frame: &mut Frame, state: &mut AppState) {
    match state.screen {
        Screen::Welcome => welcome::render(frame),
        Screen::Config => config::render(frame, state),
        Screen::Chat => chat::render(frame, state),
    }
}
