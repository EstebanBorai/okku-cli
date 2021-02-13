use hyper::Uri;
use std::str::FromStr;
use url::Url;

pub trait FromUrl {
    fn from_url(url: Url) -> Uri;
}

impl FromUrl for Uri {
    fn from_url(url: Url) -> Uri {
        Uri::from_str(url.to_string().as_str()).unwrap()
    }
}
