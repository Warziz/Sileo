extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, StoreOption};

use crate::utils::connection::ConnectionMethod;

pub struct CliOptions {
    pub method : Option<ConnectionMethod>, 
    pub anonymous : bool,
    pub search : Option<String>,
    pub username : Option<String>,
    pub server_ip : Option<String>,
    pub server_port : Option<u16>,
    pub source_port : Option<u16>,
    pub destination_port : Option<u16>,
}

pub fn parse_args() -> CliOptions {

    let mut options = CliOptions {
        method: None,
        anonymous: false,
        search: None,
        username: None,
        server_ip: None,
        server_port: None,
        source_port: None,
        destination_port: None,
    };

    {
        let mut app = ArgumentParser::new();
        app.set_description("Sileo is an instant, ephemeral messaging app designed for fast, confidential, and traceless communication.");
        
        app.refer(&mut options.method)
            .add_option(&["-m","--method"], StoreOption,"Choose your connection method. By default the program will test both method if one fail. First method (hole) is with the hole punching (pure p2p). The second method is for upnp configuration.");
        
        app.refer(&mut options.anonymous)
            .add_option(&["-a","--anonymous"], StoreTrue, "Connect to someone with generate pseudonyme");
        
        app.refer(&mut options.search)
            .add_option(&["-s","--search"], StoreOption, "Search for people you want to discuss with");
        
        app.refer(&mut options.username)
            .add_option(&["-u","--username"], StoreOption, "Enter your pseudonyme or one will be generate");    
        
        app.refer(&mut options.server_ip)
            .add_option(&["-S","--server-ip"], StoreOption, "Choose your own RendezVous Server (be sure your contact use the same)");    
        
        app.refer(&mut options.server_port)
            .add_option(&["--server-port"], StoreOption, "Bind your own RendezVous Server port");
        
        app.refer(&mut options.source_port)
            .add_option(&["--source-port"], StoreOption, "Custom your source port for upnp or hole punching (50001 by default)");   
        
        app.refer(&mut options.destination_port)
            .add_option(&["-d","--destination-port"], StoreOption, "Custome your destination port for upnp or hole punching (50002 by default)");    
        app.parse_args_or_exit();    
    }

    return options;
}