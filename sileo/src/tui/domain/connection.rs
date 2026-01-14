use crate::app::{state::AppState};

#[derive(Clone, Copy, Debug)]
pub enum ConnectionMethod {
    Hole,
    Upnp,
    Both,
}

pub fn method_to_string(state: &mut AppState) -> &str {
    match state.method { 
        ConnectionMethod::Hole => "Hole Punching", 
        ConnectionMethod::Upnp => "UPnP", 
        ConnectionMethod::Both => "Both", 
    } 
}