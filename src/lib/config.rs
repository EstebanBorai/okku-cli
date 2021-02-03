use std::env::Args;

#[derive(Debug)]
pub struct Config {
  username: String,
  password: String,
}

impl Config {
  pub fn from_args(mut args: Args) -> Result<Self, &'static str> {
    if args.len() < 3 {
      return Err("Not enough arguments. Execute okku <username> <password>");
    }

    // Skip first argument from 'Args' collection
    args.next();

    Ok(Self {
      username: args.next().unwrap(),
      password: args.next().unwrap(),
    })
  }
}
