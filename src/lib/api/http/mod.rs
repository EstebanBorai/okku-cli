use anyhow::{Error, Result};
use http_auth_basic::Credentials as AuthBasicCredentials;
use hyper::{body::HttpBody, client::HttpConnector};
use hyper::{Body, Client, Request, StatusCode, Uri};
use hyper_tls::HttpsConnector;
use responses::{LoginResponse, MeResponse};
use std::str::FromStr;

use crate::anyhowize;
use crate::config::Config;

use super::utils::from_json;

pub mod responses;

pub struct Api {
    http_client: Client<HttpsConnector<HttpConnector>>,
    server_address: Uri,
}

impl Api {
    pub fn new(config: &Config) -> Self {
        let http_client = Client::builder().build(HttpsConnector::new());

        Self {
            http_client,
            server_address: Uri::from_str(config.server_address.as_str()).unwrap(),
        }
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<LoginResponse> {
        let uri = Uri::from_str(&format!(
            "http://{}/api/v1/auth/login",
            self.server_address.to_string()
        ))
        .unwrap();

        let request = Request::builder()
            .header(
                "Authorization",
                AuthBasicCredentials::new(username, password).as_http_header(),
            )
            .uri(uri)
            .method("GET")
            .body(Body::empty())
            .unwrap();

        let mut response = self.http_client.request(request).await?;

        if response.status() != StatusCode::OK {
            match response.status() {
                StatusCode::FORBIDDEN => {
                    return Err(Error::msg("Invalid credentials provided"));
                }
                _ => {
                    return Err(Error::msg(format!(
                        "Failed to login, received response with status code: {}",
                        response.status().as_u16()
                    )));
                }
            }
        }

        match response.body_mut().data().await {
            Some(bytes) => {
                let bytes = bytes.map_err(|e| anyhowize!(e))?;
                let bytes = bytes.to_vec();

                from_json::<'_, LoginResponse>(&bytes)
            }
            None => {
                // We shouldn't get here because if the HTTP status code
                // is different to 200 we exit the function early
                Err(Error::msg("Unexpected response from server. Response from server was 200 but no body were attached"))
            }
        }
    }

    pub async fn me(&self, token: &str) -> Result<MeResponse> {
        let uri = Uri::from_str(&format!(
            "http://{}/api/v1/auth/me",
            self.server_address.to_string()
        ))
        .unwrap();

        let request = Request::builder()
            .header("Authorization", format!("Bearer {}", token))
            .uri(uri)
            .method("GET")
            .body(Body::empty())
            .unwrap();

        let mut response = self.http_client.request(request).await?;

        if response.status() != StatusCode::OK {
            match response.status() {
                StatusCode::FORBIDDEN => {
                    return Err(Error::msg("Invalid token provided"));
                }
                _ => {
                    return Err(Error::msg(format!(
                        "Failed to fetch profile details, received response with status code: {}",
                        response.status().as_u16()
                    )));
                }
            }
        }

        match response.body_mut().data().await {
            Some(bytes) => {
                let bytes = bytes.map_err(|e| anyhowize!(e))?;
                let bytes = bytes.to_vec();

                from_json::<'_, MeResponse>(&bytes)
            }
            None => {
                // We shouldn't get here because if the HTTP status code
                // is different to 200 we exit the function early
                Err(Error::msg("Unexpected response from server. Response from server was 200 but no body were attached"))
            }
        }
    }
}
