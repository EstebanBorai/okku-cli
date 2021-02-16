use anyhow::{Error, Result};
use hyper::Uri;
use std::env::{var, Args};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug)]
pub struct Config {
    pub(crate) server_address: String,
    pub(crate) chat_id: Uuid,
    pub(crate) username: String,
    pub(crate) password: String,
}

impl Config {
    pub fn from_args(mut args: Args) -> Result<Self> {
        if args.len() < 3 {
            // We check on 3 arguments because the first element
            // is the path to the executable
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

        let chat_id: Uuid = match var("OKKU_CHAT") {
            Ok(chat_id) => Uuid::from_str(chat_id.as_str())
                .expect("\"OKKU_CHAT\" environment variable is not a valid UUID"),
            Err(_) => {
                panic!("Missing \"OKKU_CHAT\" environment variable");
            }
        };

        if let Err(e) = Uri::from_str(&format!("http://{}", server_address)) {
            return Err(Error::msg(e.to_string()));
        }

        Ok(Self {
            username: args.next().unwrap(),
            password: args.next().unwrap(),
            server_address,
            chat_id,
        })
    }
}
