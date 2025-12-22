extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, Store};


fn main() {

    let mut method = "both".to_string();
    let mut anonymous = true;
    let mut search = "none".to_string();
    let mut username = "none".to_string();
    let mut server_ip = "none".to_string();
    let mut server_port = 00000;
    let mut source_port = 00000;
    let mut destination_port = 000000;

    {
        let mut app = ArgumentParser::new();
        app.set_description("Sileo is an instant, ephemeral messaging app designed for fast, confidential, and traceless communication.");
        app.refer(&mut method)
            .add_option(&["-m","--method"], Store,"Choose your connection method. By default the program will test both method if one fail. First method (hole) is with the hole punching (pure p2p). The second method is for upnp configuration.");
        
        app.refer(&mut anonymous)
            .add_option(&["-a","--anonymous"], StoreTrue, "Connect to someone with false pseudonyme");
        
        app.refer(&mut search)
            .add_option(&["-s","--search"], Store, "Search for people you want to discuss with");
        
        app.refer(&mut username)
            .add_option(&["-u","--username"], Store, "Enter your pseudonyme or one will be generate");    
        
        app.refer(&mut server_ip)
            .add_option(&["-S","--server-ip"], Store, "Choose your own RendezVous Server (be sure your contact use the same)");    
        
        app.refer(&mut server_port)
            .add_option(&["--server-port"], Store, "Bind your own RendezVous Server port");
        
        app.refer(&mut source_port)
            .add_option(&["--source-port"], Store, "Custom your source port for upnp or hole punching (50001 by default)");   
        
        app.refer(&mut destination_port)
            .add_option(&["-d","--destination-port"], Store, "Custome your destination port for upnp or hole punching (50002 by default)");    
        app.parse_args_or_exit();    
    }

}
