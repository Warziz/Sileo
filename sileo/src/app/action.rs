use crate::app::state::{AppState, Screen};
use crate::messaging::utils::connection::ConnectionMethod;
use crate::messaging::client::MessagingClient;

impl AppState {

    pub fn start_chat(&mut self) {
        
        match self.build_config() {
            Ok(config) => {
                let mut client = MessagingClient::new(
                    config,
                    self.tx_from_backend.clone(),
                    self.rx_to_backend.take().expect("Backend init failed"),
                ).expect("Backend init failed");

                client.start();
                self.screen = Screen::Chat;
            }
            Err(err) => {
                self.error_message = Some(err);
            }
        }
    }

    pub fn quit_to_welcome(&mut self) {
        self.screen = Screen::Welcome;
    }


    pub fn toggle_anonymous(&mut self) {
        self.anonymous = !self.anonymous;
    }

    pub fn cycle_method(&mut self) {
        self.method = match self.method {
            ConnectionMethod::Hole => ConnectionMethod::Upnp,
            ConnectionMethod::Upnp => ConnectionMethod::Both,
            ConnectionMethod::Both => ConnectionMethod::Hole,
        };
    }


    pub fn submit_message(&mut self){
        
        let msg = self.input.clone();

        // send to backend
        let _ = self.tx_to_backend.send(msg.clone());
  
        // print it to tui
        self.messages.push(format!("You: {}", msg));

        self.input.clear();
        self.character_index = 0;
    }

    pub fn poll_backend(&mut self){
        while let Ok(msg) = self.rx_from_backend.try_recv() {
            self.messages.push(format!("Peer: {}", msg));
        }
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    pub fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }
    
    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }
}
