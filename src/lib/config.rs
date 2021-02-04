use anyhow::{Context, Error, Result};
use hyper::Uri;
use std::env::{var, Args};
use std::str::FromStr;

#[derive(Debug)]
pub struct Config {
    pub(crate) server_address: Uri,
    pub(crate) username: String,
    pub(crate) password: String,
}

impl Config {
    pub fn from_args(mut args: Args) -> Result<Self> {
        if args.len() < 3 {
            return Err(Error::msg(
                "Not enough arguments. Execute okku_cli <username> <password>",
            ));
        }

        // Skip first argument from 'Args' collection
        args.next();

        let server_addr = match var("OKKU_HOST") {
            Ok(address) => address,
            Err(_) => {
                eprintln!("Missing \"OKKU_HOST\" environment variable, using default instead");

                String::from("0.0.0.0:3000")
            }
        };

        let server_address = Uri::from_str(&server_addr)
            .with_context(|| "The value for \"OKKU_HOST\" is not a valid Socket Address")?;

        Ok(Self {
            username: args.next().unwrap(),
            password: args.next().unwrap(),
            server_address,
        })
    }
}
