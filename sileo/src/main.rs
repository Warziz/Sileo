mod utils;
mod config;

use utils::arg::parse_args;
use config::config::Config;

fn main() {

    let cli_opts = parse_args();

    let config = Config::from_cli(cli_opts)
        .expect("Invalid configuration");

    println!("Runtime config: {:?}", config);

}
