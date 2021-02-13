use anyhow::{Context, Error, Result};
use http_auth_basic::Credentials;
use hyper::{body::HttpBody, client::HttpConnector};
use hyper::{Body, Client as HyperClient, Request, Response, Uri};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use std::str::FromStr;
use url::Url;

use crate::anyhowize;
use crate::api::utils::from_json;
use crate::api::LoginResponse;

use super::error::HttpError;
use super::from_url::FromUrl;

pub struct Client {
    pub(crate) token: Option<String>,
    http_client: HyperClient<HttpsConnector<HttpConnector>>,
    api_address: Url,
}

impl Client {
    pub fn new(api_address: &str) -> Result<Self> {
        let api_address = Url::from_str(api_address).context("Invalid API address provided")?;
        let http_client = HyperClient::builder().build(HttpsConnector::new());

        Ok(Self {
            http_client,
            api_address,
            token: None,
        })
    }

    pub async fn get<T>(&self, uri: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let (request, url, _) = self.make_request(uri, "GET", None)?;

        let response = self
            .http_client
            .request(request)
            .await
            .map_err(|e| anyhowize!(e))?;

        HttpError::is_error(url, &response)?;

        Client::parse_body(response).await
    }

    pub async fn retrieve_token(&mut self, user_id: &str, password: &str) -> Result<LoginResponse> {
        let (url, uri) = self.make_uri("api/v1/auth/login");
        let credentials = Credentials::new(user_id, password);
        let request = Request::builder()
            .header("authorization", credentials.as_http_header())
            .uri(uri)
            .method("GET")
            .body(Body::empty())
            .context("Unable to build HTTP request")?;

        let response = self
            .http_client
            .request(request)
            .await
            .context("An error ocurred performing an HTTP request")?;

        HttpError::is_error(url, &response)?;

        let login_response = Client::parse_body::<LoginResponse>(response).await?;

        Ok(login_response)
    }

    fn make_request(
        &self,
        path: &str,
        method: &str,
        body: Option<Body>,
    ) -> Result<(Request<Body>, Url, Uri)> {
        if matches!(self.token, None) {
            return Err(Error::msg("Missing authorization token"));
        }

        let (url, uri) = self.make_uri(path);
        let request = Request::builder().uri(uri.clone()).method(method).header(
            "authorization",
            format!("Bearer {}", self.token.clone().unwrap()),
        );

        let request: Result<Request<Body>> = match body {
            Some(body) => Ok(request.body(body)?),
            None => Ok(request.body(Body::empty())?),
        };

        Ok((request.unwrap(), url, uri))
    }

    fn make_uri(&self, path: &str) -> (Url, Uri) {
        let mut url = self.api_address.clone();

        url.set_path(path);

        (url.clone(), Uri::from_url(url))
    }

    async fn parse_body<T>(mut response: Response<Body>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let body = response.body_mut().data().await;

        if body.is_none() {
            return Err(Error::msg("Empty body provided"));
        }

        let bytes = body.unwrap().context("Invalid body content")?;
        let payload = from_json::<T>(&bytes)?;

        Ok(payload)
    }
}
