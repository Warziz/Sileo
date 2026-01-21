use crossterm::event::{KeyEvent, KeyCode, KeyEventKind};
use crate::app::state::{AppState, InputMode, Screen};

pub fn handle(key: KeyEvent, state: &mut AppState) {
    match state.input_mode {
        InputMode::Normal => match key.code {
            KeyCode::Char('e') => state.input_mode = InputMode::Editing,
            KeyCode::Char('q') => state.screen = Screen::Welcome,
            KeyCode::Up => {
                state.vertical_scroll = state.vertical_scroll.saturating_sub(10);
            },
            KeyCode::Down => {
                let height = state.last_chat_height.unwrap_or(10);
                    state.vertical_scroll = (state.vertical_scroll + height).min(
                    state.messages.len().saturating_sub(1)
                );
            },
            _ => {}
        },
        InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Enter => state.submit_message(),
            KeyCode::Char(c) => state.enter_char(c),
            KeyCode::Backspace => state.delete_char(),
            KeyCode::Left => state.move_cursor_left(),
            KeyCode::Right => state.move_cursor_right(),
            KeyCode::Esc => state.input_mode = InputMode::Normal,
            KeyCode::Up => {
                state.vertical_scroll = state.vertical_scroll.saturating_sub(10);
            },
            KeyCode::Down => {
                let height = state.last_chat_height.unwrap_or(10);
                    state.vertical_scroll = (state.vertical_scroll + height).min(
                    state.messages.len().saturating_sub(1)
                );
            },
            _ => {}
        },


        _ => {}
    }
}
