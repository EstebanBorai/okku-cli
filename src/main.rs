use anyhow::Context;
use std::env;
use std::process;

use okku::{run, Config};

#[tokio::main]
async fn main() {
    let config = Config::from_args(env::args()).unwrap_or_else(|msg| {
        eprint!("{}", msg);
        process::exit(1);
    });

    match run(&config)
        .await
        .with_context(|| "Okku CLI had an issue and terminated its process!")
    {
        Ok(_) => println!("Okku session exited with success!"),
        Err(e) => eprintln!("{:?}", e),
    }
}
