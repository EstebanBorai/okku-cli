use anyhow::{Context, Error, Result};
use hyper::StatusCode;
use std::str::FromStr;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, WebSocketStream};
use url::Url;

pub struct WebSocket {
    pub(crate) stream: WebSocketStream<TcpStream>,
}

impl WebSocket {
    pub async fn new(server_address: &str, token: &str) -> Result<Self> {
        let api_url = WebSocket::make_ws_address(server_address, token)?;
        let (stream, response) = connect_async(api_url.as_str()).await.unwrap();

        if response.status() != StatusCode::SWITCHING_PROTOCOLS {
            return Err(Error::msg(format!(
                "Failed to connect to Okku's chat WebSocket! Received response with status code {}",
                response.status()
            )));
        }

        Ok(Self { stream: stream })
    }

    fn make_ws_address(server_address: &str, token: &str) -> Result<String> {
        let mut url = Url::from_str(server_address).context("Invalid server address provided")?;

        match url.scheme() {
            "https" => {
                url.set_scheme("wss").unwrap();
            }
            "http" => {
                url.set_scheme("ws").unwrap();
            }
            _ => {
                return Err(Error::msg("Invalid server address provided"));
            }
        }

        Ok(format!(
            "{}api/v1/chats?token={}&frontend=terminal",
            url.to_string(),
            token = token
        ))
    }
}
