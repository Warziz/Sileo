use crate::messaging::utils::connection::ConnectionMethod;
use crate::messaging::config::Config;

use std::sync::mpsc::{Receiver, Sender, channel};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Welcome,
    Config,
    Chat
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Editing,
}


pub struct AppState {
    pub input: String,
    pub character_index: usize,
    pub input_mode: InputMode,
    pub messages: Vec<String>,
    pub screen: Screen,

    pub method: ConnectionMethod,
    pub anonymous: bool,
    pub search: String,
    pub username: String,
    pub server_ip: String,
    pub server_port: String,
    pub source_port: String,
    pub destination_port: String,

    pub selected_field: usize,
    pub config: Option<Config>,
    pub error_message: Option<String>,

    pub tx_to_backend: Sender<String>,
    pub rx_from_backend: Receiver<String>,
    pub tx_from_backend: Sender<String>,
    pub rx_to_backend: Option<Receiver<String>>
}

pub enum ConfigField {
    Method,
    Anonymous,
    Search,
    Username,
    ServerIp,
    ServerPort,
    SourcePort,
    DestinationPort,
    Spacer,
    Start,
    Quit,
}

pub const CONFIG_FIELDS: [ConfigField; 11] = [
    ConfigField::Method,
    ConfigField::Anonymous,
    ConfigField::Search,
    ConfigField::Username,
    ConfigField::ServerIp,
    ConfigField::ServerPort,
    ConfigField::SourcePort,
    ConfigField::DestinationPort,
    ConfigField::Spacer,
    ConfigField::Start,
    ConfigField::Quit,
];


impl AppState {
    pub fn new(
        tx_to_backend: Sender<String>,
        rx_from_backend: Receiver<String>,
        tx_from_backend: Sender<String>,
        rx_to_backend: Receiver<String>
    ) -> Self {

        
        Self {
            screen: Screen::Welcome,
            input_mode: InputMode::Normal,

            input: String::new(),
            character_index: 0,
            messages: Vec::new(),

            method: ConnectionMethod::Hole,
            anonymous: true,
            search: String::new(),
            username: String::new(),
            server_ip: String::new(),
            server_port: String::new(),
            source_port: String::new(),
            destination_port: String::new(),

            config: None,
            error_message: None,
            selected_field: 11,

            tx_to_backend,
            rx_from_backend,
            tx_from_backend,
            rx_to_backend: Some(rx_to_backend),

        }
    }

    pub fn build_config(&self) -> Result<Config, String> {

        let server_port = self.server_port
            .parse::<u16>()
            .map_err(|_| "Invalid server port")?;

        let source_port = self.source_port
            .parse::<u16>()
            .map_err(|_| "Invalid source port")?;

        let destination_port = self.destination_port
            .parse::<u16>()
            .map_err(|_| "Invalid destination port")?;

        let username = if self.username.trim().is_empty() {
            None
        } else {
            Some(self.username.clone())
        };

        Config::validate(
            self.method,
            self.anonymous,
            username,
            self.server_ip.clone(),
            server_port,
            source_port,
            destination_port,
        )
    }

}
