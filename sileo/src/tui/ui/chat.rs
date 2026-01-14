use ratatui::{
    Frame,
    layout::{Constraint, Layout, Position},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    style::{Style, Color},
};
use crate::app::state::{AppState, InputMode};

pub fn render(frame: &mut Frame, state: &mut AppState) {
    let layout = Layout::vertical([
        Constraint::Min(1),
        Constraint::Length(3),
        Constraint::Length(1),
    ]).split(frame.area());

    let messages_area = layout[0];
    let input_area = layout[1];
    let help_area = layout[2];

    let messages: Vec<ListItem> = state.messages
        .iter()
        .map(|m| ListItem::new(m.clone()))
        .collect();

    let messages = List::new(messages)
        .block(Block::new().borders(Borders::ALL).title("Chat"));

    frame.render_widget(messages, messages_area);

    let input = Paragraph::new(state.input.as_str())
        .style(match state.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::new().borders(Borders::ALL).title("Message"));

    frame.render_widget(input, input_area);

    if state.input_mode == InputMode::Editing {
        frame.set_cursor_position(Position::new(
            input_area.x + state.character_index as u16 + 1,
            input_area.y + 1,
        ));
    }

    let help = Paragraph::new(match state.input_mode {
        InputMode::Normal => "Press 'e' to write, 'q' to quit",
        InputMode::Editing => "Enter = send | Esc = cancel",
    });

    frame.render_widget(help, help_area);
}
