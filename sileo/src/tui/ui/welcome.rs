use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout}, 
    style::{Color, Modifier, Style}, 
    text::{Line, Span}, 
    widgets::{Block, Borders, Paragraph}
};

//Function for render welcome menu
pub fn render (frame: &mut Frame) {
        
        let layout = Layout::vertical([
            Constraint::Min(1),
            Constraint::Length(3),
        ])
            .margin(1)
            .split(frame.area());
        
        let main_area = layout[0];
        let footer_area = layout[1];

        let title = vec![
            Line::from(" ███████╗ ██╗ ██╗      ███████╗  ██████╗ "),
            Line::from(" ██╔════╝ ██║ ██║      ██╔════╝ ██╔═══██╗"),
            Line::from(" ███████╗ ██║ ██║      █████╗   ██║   ██║"),
            Line::from(" ╚════██║ ██║ ██║      ██╔══╝   ██║   ██║"),
            Line::from(" ███████║ ██║ ███████╗ ███████╗ ╚██████╔╝"),
            Line::from(" ╚══════╝ ╚═╝ ╚══════╝ ╚══════╝  ╚═════╝ "),
            Line::from("E2EE, Ephemeral & Peer-to-Peer Messaging"),
            Line::from("========================================="),
            ];

        frame.render_widget(
            Paragraph::new(title)
            .alignment(Alignment::Center)
            .block(
                Block::new()
                .borders(Borders::ALL)
                .title("v0.0.0")
                .style(
                    Style::default()
                        .fg(Color::Blue)
                        .add_modifier(Modifier::BOLD),
                ),
            ),
            main_area,
        );


        let footer_text = Line::from(vec![
            Span::styled("[ENTER]", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" Start  "),
            Span::styled("[Q]", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw(" Quit"),
        ]);

        frame.render_widget(
            Paragraph::new(footer_text)
            .alignment(Alignment::Center)
            .block(
                Block::new()
                    .borders(Borders::TOP)
                    .style(Style::default().fg(Color::Gray)),
            ), 
            footer_area);

    }