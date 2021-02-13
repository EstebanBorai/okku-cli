use anyhow::{Error, Result};
use uuid::Uuid;

use crate::config::Config;

use super::Client;
use super::{FetchChatMessagesResponse, MeResponse};

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

    /// Fetches: GET /api/v1/auth/me
    pub async fn auth_me(&self) -> Result<MeResponse> {
        self.client.get::<MeResponse>("api/v1/auth/me").await
    }

    /// Fetches: GET /api/v1/chats/:chat_id/messages
    pub async fn chat_messages(&self, chat_id: &Uuid) -> Result<FetchChatMessagesResponse> {
        self.client.get::<FetchChatMessagesResponse>(&format!("api/v1/chats/{}/messages", chat_id.to_string())).await
    }
}
