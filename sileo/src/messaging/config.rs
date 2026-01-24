
use std::net::IpAddr;

use crate::messaging::utils::connection::ConnectionMethod;
use crate::messaging::utils::user::generate_username;


/// Holds the configuration parameters required to initialize the client.
#[derive(Debug)]
pub struct Config {
    /// Selected peer connection method.
    pub method: ConnectionMethod,
    /// Enables anonymous mode when set to `true`.
    pub anonymous: bool,
    /// Username used to identify the client.
    pub username: String,
    /// IP address of the rendezvous server.
    pub server_ip: IpAddr,
    /// Port of the rendezvous server.
    pub server_port: u16,
    /// Local UDP source port.
    pub source_port: u16,
    /// Destination port used for peer-to-peer communication.
    pub destination_port: u16,
}

impl Config {

    /// Validates user-provided parameters and constructs a `Config`.
    ///
    /// This function ensures that all required fields are valid and
    /// enforces configuration invariants such as non-zero ports and
    /// proper username handling.
    ///
    /// # Arguments
    ///
    /// * `method` - The connection method to use.
    /// * `anonymous` - Whether to enable anonymous mode.
    /// * `username` - Optional username, required if anonymous mode is disabled.
    /// * `server_ip` - IP address of the rendezvous server as a string.
    /// * `server_port` - Port of the rendezvous server.
    /// * `source_port` - Local UDP source port.
    /// * `destination_port` - Destination port for peer-to-peer communication.
    ///
    /// # Returns
    ///
    /// On success, returns a validated `Config`.
    /// On failure, returns a descriptive error message.
    pub fn validate(
        method: ConnectionMethod,
        anonymous: bool,
        username: Option<String>,
        server_ip: String,
        server_port: u16,
        source_port: u16,
        destination_port: u16,
    ) -> Result<Self, String> {
        
        let server_ip = server_ip
            .parse::<IpAddr>()
            .map_err(|_| "Invalid server IP")?;

        if server_port == 0 {
            return Err("Invalid server port".into());
        }

        if source_port == 0 {
            return Err("Invalid source port".into());
        }

        if destination_port == 0 {
            return Err("Invalid destination port".into());
        }

        let username = if anonymous {
            generate_username()
        } else {
            username.ok_or("Username required when anonymous is disabled")?
        };

        Ok(Self {
            method,
            anonymous,
            username,
            server_ip,
            server_port,
            source_port,
            destination_port,
        })
    }
}