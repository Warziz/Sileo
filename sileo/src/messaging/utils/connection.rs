use std::str::FromStr;

use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize,Debug, Clone, Copy)]
pub enum ConnectionMethod {
    Hole,
    Upnp,
    Both,
}

impl FromStr for ConnectionMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err>{
        match s.to_lowercase().as_str() {
            "hole" => Ok(ConnectionMethod::Hole),
            "upnp" => Ok(ConnectionMethod::Upnp),
            "both" => Ok(ConnectionMethod::Both),
            _ => Err(format!(
                "Invalid method '{}'. Use hole | upnp | both", s)),
        }
    }
}