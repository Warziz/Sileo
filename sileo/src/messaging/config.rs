
use std::net::IpAddr;

use crate::messaging::utils::connection::ConnectionMethod;
use crate::messaging::utils::user::generate_username;

#[derive(Debug)]
pub struct Config {
    pub method: ConnectionMethod,
    pub anonymous: bool,
    pub username: String,
    pub server_ip: IpAddr,
    pub server_port: u16,
    pub source_port: u16,
    pub destination_port: u16,
}

impl Config {
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