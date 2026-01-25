use ratatui::widgets::{Block, Borders, ListItem, List};
use ratatui::style::{Style, Color, Modifier};
use ratatui::Frame;

use crate::app::state::AppState;
use crate::tui::domain::connection::method_to_string;

 //function for render config menu
pub fn render(frame: &mut Frame, state: &mut AppState) {
        let area = frame.area();


        let block = Block::new()
            .title("Configuration")
            .borders(Borders::ALL);

        let items = vec![
            format!("Method           : {}", method_to_string(state)),
            format!("Anonymous        : {}", if state.anonymous { "ON" } else { "OFF" }),
            format!("Search           : {}", state.search),
            format!("Username         : {}", state.username),
            format!("Server IP        : {}", state.server_ip),
            format!("Server Port      : {}", state.server_port),
            format!("Source Port      : {}", state.source_port),
            format!("Destination Port : {}", state.destination_port),
            String::from(""),
            String::from("  ▶ Start"),
            String::from("  ⏻ Quit"),
        ];

        let list_items: Vec<ListItem> = items
            .iter()
            .enumerate()
            .map(|(i, item)|{
                if i == state.selected_field {
                    ListItem::new(item.clone()).style(
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    ListItem::new(item.clone())
                }
            })
            .collect();

        let list = List::new(list_items)
            .block(block)
            .highlight_style(
                Style::default()
                    .bg(Color::Blue)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            );


        frame.render_widget(list,area,);
}