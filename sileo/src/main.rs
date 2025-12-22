extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, StoreOption};

struct Options {
    method : Option<String>, 
    anonymous : bool,
    search : Option<String>,
    username : Option<String>,
    server_ip : Option<String>,
    server_port : Option<u16>,
    source_port : Option<u16>,
    destination_port : Option<u16>,
}


fn main() {

    let mut options = Options {
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
            .add_option(&["-a","--anonymous"], StoreTrue, "Connect to someone with false pseudonyme");
        
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

    println!("Method: {:?}", options.method);
    println!("Anonymous: {}", options.anonymous);
    println!("Server IP: {:?}", options.server_ip);

}
