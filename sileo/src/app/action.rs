
use std::thread;

use crate::app::state::{AppState, Screen};
use crate::messaging::utils::connection::ConnectionMethod;
use crate::messaging::client::MessagingClient;
use crate::messaging::utils::event::BackendEvent;

impl AppState {

    /// Builds the client configuration and starts the chat backend.
    ///
    /// This function validates the current UI state, switches to the chat
    /// screen, and spawns a background thread responsible for initializing
    /// the messaging backend and handling network communication.
    pub fn start_chat(&mut self) {
        
        match self.build_config() {
            Ok(config) => {

                self.screen = Screen::Chat;

                let tx_logs = self.tx_from_backend.clone();
                let rx_backend = self.rx_to_backend.take().expect("Backend already started");

                thread::spawn(move || {
                    tx_logs.send(BackendEvent::Log("Backend starting...".to_string())).ok();

                    match MessagingClient::new(config, tx_logs.clone(), rx_backend){
                        Ok(mut client) => {
                            tx_logs.send(BackendEvent::Log("Connected to peer".to_string())).ok();
                            client.start();
                        }
                        Err(err) => {
                            tx_logs.send(BackendEvent::Error(err.to_string())).ok();
                        }
                    }

                });
            }
            Err(err) => {
                self.error_message = Some(err);
            }
        }
    }

    
    /// Returns to the welcome screen.
    pub fn quit_to_welcome(&mut self) {
        self.screen = Screen::Welcome;
    }


    /// Toggles anonymous mode on or off.
    pub fn toggle_anonymous(&mut self) {
        self.anonymous = !self.anonymous;
    }


    /// Cycles through the available connection methods.
    pub fn cycle_method(&mut self) {
        self.method = match self.method {
            ConnectionMethod::Hole => ConnectionMethod::Upnp,
            ConnectionMethod::Upnp => ConnectionMethod::Both,
            ConnectionMethod::Both => ConnectionMethod::Hole,
        };
    }

    /// Sends the current input message to the backend and updates the UI.
    ///
    /// The message is forwarded to the backend for transmission and
    /// immediately displayed in the chat history.
    pub fn submit_message(&mut self){
        
        let msg = self.input.clone();

        let was_at_bottom =
        self.vertical_scroll + self.visible_height >= self.messages.len();

        // send to backend
        let _ = self.tx_to_backend.send(msg.clone());
  
        // print it to tui
        self.messages.push(format!("You: {}", msg));

        if was_at_bottom {
        self.vertical_scroll = self
            .messages
            .len()
            .saturating_sub(self.visible_height);
        }

        self.input.clear();
        self.character_index = 0;
    }

    /// Polls backend events and updates the UI accordingly.
    ///
    /// This function processes incoming backend events such as logs,
    /// peer messages, connection notifications, and errors.
    pub fn poll_backend(&mut self){
        while let Ok(event) = self.rx_from_backend.try_recv() {

            let was_at_bottom =
            self.vertical_scroll + self.visible_height >= self.messages.len();

            match event {
                BackendEvent::Log(msg) => {
                    self.messages.push(format!("[INFO]: {}",msg));
                }

                BackendEvent::PeerConnected { username } => {
                    self.peer_username = Some(username.clone());
                    self.messages.push(format!("[INFO]: Connected to {}", username));
                }

                BackendEvent::PeerMessage { username, message } => {
                    self.messages.push(format!("{}: {}", username, message));
                }

                BackendEvent::Error(err) => {
                    self.messages.push(format!("[ERROR]: {}", err));
                }
            }

            if was_at_bottom {
                self.vertical_scroll = self
                    .messages
                    .len()
                    .saturating_sub(self.visible_height);
        }

        }
    }


    /// Clamps the cursor position to a valid character index.
    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }


    /// Returns the byte index corresponding to the current cursor position.
    pub fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }
    
    /// Moves the cursor one character to the left.
    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }


    /// Moves the cursor one character to the right.
    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    /// Inserts a character at the current cursor position.
    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    
    /// Deletes the character immediately to the left of the cursor.
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
