use crate::{app::state::AppState};
use crate::app::state::Action;
use crossterm::event::{KeyEvent, KeyCode,KeyEventKind};

//function for handle input
pub fn handle(key: KeyEvent, state: &mut AppState) {
    if key.kind != KeyEventKind::Press {
        return;
    }

    match key.code {
        KeyCode::Enter => {
            state.pending_action = Some(Action::GoToConfig);
        }

        KeyCode::Char('q') => {
            state.pending_action = Some(Action::Quit);
        }

        _ => {}
    }
}
