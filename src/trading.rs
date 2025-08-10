use crate::alpaca::AlpacaTradingClient;
use crate::error::{OrcastError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleLegOrder {
    pub symbol: String,
    pub qty: String,
    pub side: String,
    pub order_type: String,
    pub time_in_force: String,
    pub limit_price: Option<String>,
    pub stop_price: Option<String>,
}

pub async fn submit_single_leg_order(
    client: &AlpacaTradingClient,
    order: SingleLegOrder,
) -> Result<()> {
    let mut body = serde_json::json!({
        "symbol": order.symbol,
        "qty": order.qty,
        "side": order.side,
        "type": order.order_type,
        "time_in_force": order.time_in_force,
    });
    if let Some(lp) = order.limit_price { body["limit_price"] = serde_json::json!(lp); }
    if let Some(sp) = order.stop_price { body["stop_price"] = serde_json::json!(sp); }

    let req = client
        .build_create_order_request(body)
        .map_err(|e| OrcastError::Config(format!("request build: {e}")))?;

    // NOTE: send not implemented in scaffold
    let _ = req;
    Err(OrcastError::NotImplemented("HTTP send layer"))
}


