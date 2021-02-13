mod client;
mod responses;

pub use client::*;
pub use responses::*;

// use anyhow::{Error, Result};
// use http_auth_basic::Credentials as AuthBasicCredentials;
// use hyper::{body::HttpBody, client::HttpConnector};
// use hyper_tls::HttpsConnector;
// use responses::{LoginResponse, MeResponse};
// use std::str::FromStr;

// use crate::anyhowize;
// use crate::api::http::client::Client;
// use crate::config::Config;

// use super::utils::from_json;

// pub struct Api {
//     client: Client,
// }

// impl Api {
//     pub async fn new(config: &Config) -> Result<Self> {
//         let mut client = Client::new(config.server_address.as_str())?;

//         client
//             .retrieve_token(config.username.as_str(), config.password.as_str())
//             .await?;

//         Ok(Self { client })
//     }

//     pub async fn me(&self, token: &str) -> Result<MeResponse> {
//         let response = self.client.get("api/v1/auth/me");

//         let request = Request::builder()
//             .header("Authorization", format!("Bearer {}", token))
//             .uri(uri)
//             .method("GET")
//             .body(Body::empty())
//             .unwrap();

//         let mut response = self.http_client.request(request).await?;

//         if response.status() != StatusCode::OK {
//             match response.status() {
//                 StatusCode::FORBIDDEN => {
//                     return Err(Error::msg("Invalid token provided"));
//                 }
//                 _ => {
//                     return Err(Error::msg(format!(
//                         "Failed to fetch profile details, received response with status code: {}",
//                         response.status().as_u16()
//                     )));
//                 }
//             }
//         }

//         match response.body_mut().data().await {
//             Some(bytes) => {
//                 let bytes = bytes.map_err(|e| anyhowize!(e))?;
//                 let bytes = bytes.to_vec();

//                 from_json::<'_, MeResponse>(&bytes)
//             }
//             None => {
//                 // We shouldn't get here because if the HTTP status code
//                 // is different to 200 we exit the function early
//                 Err(Error::msg("Unexpected response from server. Response from server was 200 but no body were attached"))
//             }
//         }
//     }
// }
