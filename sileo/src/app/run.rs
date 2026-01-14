use crate::tui::ui;
use crate::tui::input;
use crate::app::App;
use ratatui::DefaultTerminal;
use crossterm::event::{self, Event, KeyEventKind};


impl App {
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        loop {
            terminal.draw(|frame| {
                ui::render(frame, &mut self.state);
            })?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    input::handle(key, &mut self.state);
                }
            }
        }
    }
}