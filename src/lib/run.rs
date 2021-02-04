use anyhow::Result;

use crate::api::Api;
use crate::config::Config;

pub async fn run(config: &Config) -> Result<()> {
    let okku_api = Api::new(config);

    let token = okku_api
        .login(config.username.as_str(), config.password.as_str())
        .await?;

    println!("{:?}", token);
    Ok(())
}
