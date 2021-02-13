use anyhow::{Error, Result};
use hyper::body::Bytes;
use serde::de::DeserializeOwned;
use serde_json::from_slice;

use crate::anyhowize;

pub fn from_json<T>(bytes: &Bytes) -> Result<T>
where
    T: DeserializeOwned,
{
    let bytes = bytes.to_vec();
    let bytes = bytes.as_slice();
    let result = from_slice::<'_, T>(bytes).map_err(|e| anyhowize!(e));

    result
}
