use anyhow::{Error, Result};
use serde::Deserialize;
use serde_json::from_str;
use std::str::from_utf8;

use crate::anyhowize;

pub fn from_json<'d, T>(bytes: &'d [u8]) -> Result<T>
where
    T: Deserialize<'d>,
{
    let utf8_value = from_utf8(bytes).map_err(|e| anyhowize!(e))?;
    let value: T = from_str::<'d, T>(utf8_value).map_err(|e| anyhowize!(e))?;

    Ok(value)
}
