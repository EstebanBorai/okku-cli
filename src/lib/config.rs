use anyhow::{Context, Error, Result};
use hyper::Uri;
use std::env::{var, Args};
use std::str::FromStr;

#[derive(Debug)]
pub struct Config {
    pub(crate) server_address: String,
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

        let server_address = match var("OKKU_HOST") {
            Ok(address) => address,
            Err(_) => {
                eprintln!("Missing \"OKKU_HOST\" environment variable, using default instead");

                String::from("0.0.0.0:3000")
            }
        };

        if let Err(e) = Uri::from_str(&format!("http://{}", server_address)) {
            return Err(Error::msg(e.to_string()));
        }

        Ok(Self {
            username: args.next().unwrap(),
            password: args.next().unwrap(),
            server_address,
        })
    }
}
