use std::str::FromStr;

use serde::{Deserialize,Serialize};

/// Defines the supported peer connection methods.
#[derive(Deserialize,Serialize,Debug, Clone, Copy)]
pub enum ConnectionMethod {
     /// Establishes a connection using UDP hole punching only.
    Hole,
    /// Establishes a connection using UPnP port forwarding only.
    Upnp,
    /// Attempts both hole punching and UPnP.
    Both,
}

impl FromStr for ConnectionMethod {
    type Err = String;

    /// Parses a connection method from a string.
    ///
    /// The parsing is case-insensitive and accepts the following values:
    /// - `"hole"`
    /// - `"upnp"`
    /// - `"both"`
    ///
    /// # Errors
    ///
    /// Returns an error if the provided string does not match
    /// any supported connection method.
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