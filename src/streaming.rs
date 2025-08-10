use crate::error::{OrcastError, Result};

pub async fn subscribe_trade_updates() -> Result<()> {
    Err(OrcastError::NotImplemented("trading updates websocket"))
}

pub async fn subscribe_option_quotes() -> Result<()> {
    Err(OrcastError::NotImplemented("option data websocket (msgpack)"))
}


