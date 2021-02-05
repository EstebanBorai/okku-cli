use anyhow::{Error, Result};
use hyper::StatusCode;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, WebSocketStream};

pub struct WebSocket {
    pub(crate) stream: WebSocketStream<TcpStream>,
}

impl WebSocket {
    pub async fn new(server_address: &str, token: &str) -> Result<Self> {
        let api_url = format!(
            "ws://{server_address}/api/v1/chats?token={token}",
            server_address = server_address,
            token = token
        );
        let (stream, response) = connect_async(api_url.as_str()).await.unwrap();

        if response.status() != StatusCode::SWITCHING_PROTOCOLS {
            return Err(Error::msg(format!(
                "Failed to connect to Okku's chat WebSocket! Received response with status code {}",
                response.status()
            )));
        }

        Ok(Self { stream: stream })
    }
}
