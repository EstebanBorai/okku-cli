use anyhow::{Error, Result};
use hyper::{Body, Response, StatusCode};
use std::fmt::Display;
use url::Url;

#[derive(Debug)]
pub struct HttpError {
    pub status_code: StatusCode,
    pub request_url: Url,
    pub body: Option<Body>,
}

impl Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Request to {} responded with {}",
            self.request_url.to_string(),
            self.status_code
        )
    }
}

impl std::error::Error for HttpError {}

impl HttpError {
    pub fn is_error(path: Url, response: &Response<Body>) -> Result<()> {
        if response.status().as_u16() >= 300 {
            return Err(Error::new(HttpError {
                status_code: response.status(),
                request_url: path,
                // This must be implemented in the future for
                // better debugging experience
                body: None,
            }));
        }

        Ok(())
    }
}
