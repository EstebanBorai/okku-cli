use anyhow::{Error, Result};

use crate::config::Config;

use super::Client;
use super::MeResponse;

pub struct Api {
    client: Client,
}

impl Api {
    pub async fn new(config: &Config) -> Result<Self> {
        let mut client = Client::new(config.server_address.as_str())?;
        let login_response = client
            .retrieve_token(config.username.as_str(), config.password.as_str())
            .await?;

        client.token = Some(login_response.token.clone());

        Ok(Self { client })
    }

    pub fn token(&self) -> Result<String> {
        if let Some(token) = self.client.token.clone() {
            return Ok(token.to_string());
        }

        Err(Error::msg("Token is not available"))
    }

    pub async fn auth_me(&self) -> Result<MeResponse> {
        self.client.get::<MeResponse>("api/v1/auth/me").await
    }
}
