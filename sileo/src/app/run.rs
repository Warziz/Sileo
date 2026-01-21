use std::time::{Duration};

use crate::tui::ui;
use crate::tui::input;
use crate::app::App;
use crate::app::state::{Action, Screen};

use ratatui::DefaultTerminal;
use crossterm::event::{self, Event, KeyEventKind};


impl App {
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {

        loop {

            //recieve peer message
            self.state.poll_backend();

            if let Some(action) = self.state.pending_action.take() {
                match action {
                    Action::GoToConfig => {
                        self.state.screen = Screen::Config;
                    }
                    Action::Quit => {
                        break Ok(());
                    }
                }
            }


            //re-generate tui 
            terminal.draw(|frame| {
                ui::render(frame, &mut self.state);
            })?;

            if event::poll(Duration::from_millis(16))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        input::handle(key, &mut self.state);
                    }
                    
                }
            }

        }
    }
}