use crate::app::{state::AppState};
use crate::messaging::utils::connection::ConnectionMethod;

pub fn method_to_string(state: &mut AppState) -> &str {
    match state.method { 
        ConnectionMethod::Hole => "Hole Punching", 
        ConnectionMethod::Upnp => "UPnP", 
        ConnectionMethod::Both => "Both", 
    } 
}