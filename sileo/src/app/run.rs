use std::time::{Duration, Instant};

use crate::tui::ui;
use crate::tui::input;
use crate::app::App;

use ratatui::DefaultTerminal;
use crossterm::event::{self, Event, KeyEventKind};


impl App {
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {

        let tick_rate = Duration::from_millis(100);
        let mut last_tick = Instant::now();

        loop {

            //recieve peer message
            self.state.poll_backend();

            //re-generate tui 
            terminal.draw(|frame| {
                ui::render(frame, &mut self.state);
            })?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or(Duration::from_secs(0));

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    input::handle(key, &mut self.state);
                }
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
    }
}