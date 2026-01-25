use crossterm::event::{KeyEvent, KeyCode};
use crate::app::state::{AppState, InputMode, ConfigField, CONFIG_FIELDS};

pub fn handle(key: KeyEvent, state: &mut AppState) {
    match state.input_mode {
        InputMode::Normal => match key.code {
            KeyCode::Up => {
                if state.selected_field > 0 {
                    state.selected_field -= 1;
                }
            }
            KeyCode::Down => {
                if state.selected_field + 1 < CONFIG_FIELDS.len() {
                    state.selected_field += 1;
                }
            }
            KeyCode::Enter => {
                match CONFIG_FIELDS[state.selected_field] {
                    ConfigField::Start => state.start_chat(),
                    ConfigField::Quit => state.quit_to_welcome(),
                    _ => state.input_mode = InputMode::Editing,
                }
            }
            KeyCode::Esc => state.quit_to_welcome(),
            _ => {}
        },

        InputMode::Editing => match CONFIG_FIELDS[state.selected_field] {
            ConfigField::Anonymous => match key.code {
                KeyCode::Enter | KeyCode::Char(' ') => {
                    state.toggle_anonymous();
                    state.input_mode = InputMode::Normal;
                }
                KeyCode::Esc => state.input_mode = InputMode::Normal,
                _ => {}
            },

            ConfigField::Method => match key.code {
                KeyCode::Left | KeyCode::Right | KeyCode::Enter => {
                    state.cycle_method();
                    state.input_mode = InputMode::Normal;
                }
                KeyCode::Esc => state.input_mode = InputMode::Normal,
                _ => {}
            },

            _ => {
                // texte
                match key.code {
                    KeyCode::Char(c) => {
                        match CONFIG_FIELDS[state.selected_field] {
                            ConfigField::Search => state.search.push(c),
                            ConfigField::Username => state.username.push(c),
                            ConfigField::ServerIp => state.server_ip.push(c),
                            ConfigField::ServerPort => state.server_port.push(c),
                            ConfigField::SourcePort => state.source_port.push(c),
                            ConfigField::DestinationPort => state.destination_port.push(c),
                            _ => {}
                        }
                    }
                    KeyCode::Backspace => {
                        match CONFIG_FIELDS[state.selected_field] {
                            ConfigField::Search => { state.search.pop(); }
                            ConfigField::Username => { state.username.pop(); }
                            ConfigField::ServerIp => { state.server_ip.pop(); }
                            ConfigField::ServerPort => { state.server_port.pop(); }
                            ConfigField::SourcePort => { state.source_port.pop(); }
                            ConfigField::DestinationPort => { state.destination_port.pop(); }
                            _ => {}
                        }
                    }
                    KeyCode::Enter | KeyCode::Esc => {
                        state.input_mode = InputMode::Normal;
                    }
                    _ => {}
                }
            }
        }
    }
}

