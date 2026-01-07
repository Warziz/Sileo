
use std::net::IpAddr;

use crate::utils::arg::CliOptions;
use crate::utils::connection::ConnectionMethod;
use crate::utils::user::generate_username;

#[derive(Debug)]
pub struct Config {
    pub method: ConnectionMethod,
    pub username: String,
    pub server_ip: IpAddr,
    pub server_port: u16,
    pub source_port: u16,
    pub destination_port: u16,
}

impl Config {
    pub fn from_cli(cli: CliOptions) -> Result<Self, String> {
        let server_ip = cli.server_ip
            .unwrap_or("127.0.0.1".to_string())
            .parse::<IpAddr>()
            .map_err(|_| "Invalid server IP")?;

        let server_port = cli.server_port.unwrap_or(4242);

        if server_port == 0 {
            return Err("Invalid server port".into());
        }

        let source_port = cli.source_port.unwrap_or(50001);

        if source_port == 0 {
            return Err("Invalid server port".into());
        }

        let destination_port = cli.destination_port.unwrap_or(50002);

        if destination_port == 0 {
            return Err("Invalid server port".into());
        }

        let username = if cli.anonymous {
            generate_username()
        } else {
            cli.username.ok_or("Username  required when anonymous mode is disabled")?
        };

        Ok(Self {
            method: cli.method.unwrap_or(ConnectionMethod::Both),
            username,
            server_ip,
            server_port,
            source_port,
            destination_port,

        })
    }
}