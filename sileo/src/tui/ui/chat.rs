use ratatui::{
    Frame, layout::{Constraint, Layout, Position}, 
    style::{Color, Style}, 
    widgets::{Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation}
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

    //Scrolling part
    let visible_height = messages_area.height.saturating_sub(2) as usize;
    state.visible_height = visible_height;
    let total  = state.messages.len();

    let max_scroll = total.saturating_sub(visible_height);
    let scroll = state.vertical_scroll.min(max_scroll);
    let visible = &state.messages[scroll..(scroll + visible_height).min(total)];

    let items: Vec<ListItem> = visible
        .iter()
        .map(|m| ListItem::new(m.clone()))
        .collect();

    let list = List::new(items)
        .block(Block::new().borders(Borders::ALL).title("Chat"));

    frame.render_widget(list, messages_area);

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

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("▲"))
        .end_symbol(Some("▼"));

     state.scrollbar_state = state.scrollbar_state
        .position(scroll)
        .content_length(total)
        .viewport_content_length(visible_height);

    frame.render_stateful_widget(scrollbar,messages_area,&mut state.scrollbar_state);
    
}
