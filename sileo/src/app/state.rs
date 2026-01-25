use ratatui::widgets::ScrollbarState;

use crate::messaging::utils::{connection::ConnectionMethod, event::BackendEvent};
use crate::messaging::config::Config;

use std::sync::mpsc::{Receiver, Sender};

/// Represents the different screens of the application.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    /// Initial welcome screen.
    Welcome,
    /// Configuration screen.
    Config,
    /// Chat screen.
    Chat,
}

/// Represents the current input mode of the TUI.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    /// Normal navigation mode.
    Normal,
    /// Text editing mode.
    Editing,
}

/// Represents a high-level user action.
pub enum Action {
    /// Navigate to the configuration screen.
    GoToConfig,
    /// Quit the application.
    Quit,
}

/// Holds the complete state of the application.
///
/// `AppState` centralizes UI state, user input, configuration data,
/// backend communication channels, and runtime metadata.
pub struct AppState {
    /// Current text input buffer.
    pub input: String,

    /// Cursor position in characters within the input buffer.
    pub character_index: usize,

    /// Current input mode.
    pub input_mode: InputMode,

    /// Chat message history.
    pub messages: Vec<String>,

    /// Currently active screen.
    pub screen: Screen,

    /// Vertical scroll offset for the chat view.
    pub vertical_scroll: usize,

    /// Last known height of the chat view.
    pub last_chat_height: Option<usize>,

    /// Number of visible lines in the chat view.
    pub visible_height: usize,

    /// Scrollbar rendering state.
    pub scrollbar_state: ScrollbarState,

    /// Selected peer connection method.
    pub method: ConnectionMethod,

    /// Whether anonymous mode is enabled.
    pub anonymous: bool,

    /// Search input value.
    pub search: String,

    /// Username entered by the user.
    pub username: String,

    /// Rendezvous server IP address (raw input).
    pub server_ip: String,

    /// Rendezvous server port (raw input).
    pub server_port: String,

    /// Local UDP source port (raw input).
    pub source_port: String,

    /// Destination port for peer-to-peer communication (raw input).
    pub destination_port: String,

    /// Username of the connected peer, if available.
    pub peer_username: Option<String>,

    /// Index of the currently selected configuration field.
    pub selected_field: usize,

    /// Optional error message to display to the user.
    pub error_message: Option<String>,

    /// Pending high-level action requested by the UI.
    pub pending_action: Option<Action>,

    /// Channel used to send messages to the backend.
    pub tx_to_backend: Sender<String>,

    /// Channel used to receive events from the backend.
    pub rx_from_backend: Receiver<BackendEvent>,

    /// Channel used by the backend to send events.
    pub tx_from_backend: Sender<BackendEvent>,

    /// Channel used by the backend to receive outgoing messages.
    pub rx_to_backend: Option<Receiver<String>>,
}

/// Represents the editable fields in the configuration screen.
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

/// Ordered list of configuration fields used for navigation.
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
    /// Creates a new `AppState` with default values.
    ///
    /// Backend communication channels must be provided at initialization.
    pub fn new(
        tx_to_backend: Sender<String>,
        rx_from_backend: Receiver<BackendEvent>,
        tx_from_backend: Sender<BackendEvent>,
        rx_to_backend: Receiver<String>,
    ) -> Self {
        Self {
            screen: Screen::Welcome,
            input_mode: InputMode::Normal,

            input: String::new(),
            character_index: 0,
            messages: Vec::new(),
            vertical_scroll: 0,
            last_chat_height: None,
            visible_height: 0,
            scrollbar_state: ScrollbarState::new(0),

            method: ConnectionMethod::Hole,
            anonymous: true,
            search: String::new(),
            username: String::new(),
            server_ip: String::new(),
            server_port: String::new(),
            source_port: String::new(),
            destination_port: String::new(),
            peer_username: None,

            error_message: None,
            selected_field: 11,
            pending_action: None,

            tx_to_backend,
            rx_from_backend,
            tx_from_backend,
            rx_to_backend: Some(rx_to_backend),
        }
    }

    /// Builds and validates a `Config` from the current UI state.
    ///
    /// User-provided string inputs are parsed and validated before
    /// constructing the final configuration.
    pub fn build_config(&self) -> Result<Config, String> {
        let server_port = self
            .server_port
            .parse::<u16>()
            .map_err(|_| "Invalid server port")?;

        let source_port = self
            .source_port
            .parse::<u16>()
            .map_err(|_| "Invalid source port")?;

        let destination_port = self
            .destination_port
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

