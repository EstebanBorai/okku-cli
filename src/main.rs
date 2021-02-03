use std::env;
use std::process;

use okku::config::Config;

#[tokio::main]
async fn main() {
    let config = Config::from_args(env::args()).unwrap_or_else(|msg| {
        eprint!("{}", msg);
        process::exit(1);
    });

    println!("{:?}", config);
}
