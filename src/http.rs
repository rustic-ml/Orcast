use crate::error::{OrcastError, Result};
use reqwest::{Client, ClientBuilder};
use std::time::Duration;

pub fn build_http_client() -> Result<Client> {
    let client = ClientBuilder::new()
        .tcp_nodelay(true)
        .pool_max_idle_per_host(4)
        .timeout(Duration::from_millis(10_000))
        .build()
        .map_err(OrcastError::Http)?;
    Ok(client)
}


