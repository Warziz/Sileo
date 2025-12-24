mod arg;
mod config;
mod connection;

use arg::parse_args;
use config::Config;

fn main() {

    let cli_opts = parse_args();

    let config = Config::from_cli(cli_opts)
        .expect("Invalid configuration");

    println!("Runtime config: {:?}", config);

}
