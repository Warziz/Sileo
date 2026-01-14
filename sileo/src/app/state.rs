use crate::tui::domain::connection::ConnectionMethod;
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

    pub tx_backend: Sender<String>,
    pub rx_backend: Receiver<String>,

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
    pub fn new() -> Self {

        let (tx_backend, rx_backend) = channel::<String>();
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

            selected_field: 11,

            tx_backend,
            rx_backend,
        }
    }
}
