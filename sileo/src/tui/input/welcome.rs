use crate::{app::state::AppState, tui::input::Screen};
use crossterm::event::{KeyEvent, KeyCode};

//function for handle input
pub fn handle(key:KeyEvent, state: &mut AppState) {
    match key.code {
        KeyCode::Enter => {
            state.screen = Screen::Config
        }

        KeyCode::Char('q') => {
            std::process::exit(0);
        }

        _ => {}
    }
}
