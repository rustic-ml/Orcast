use crate::error::{OrcastError, Result};
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct DailyBarsRequest {
    pub symbol: String,
    pub start: String, // ISO8601 date, e.g., 2025-01-01
    pub end: String,   // ISO8601 date, e.g., 2025-06-01
    pub data_base_url: String, // e.g., https://data.alpaca.markets
}

#[derive(Debug, Deserialize, Clone)]
pub struct DailyBar {
    pub t: String, // time
    pub o: f64,
    pub h: f64,
    pub l: f64,
    pub c: f64,
    pub v: f64,
}

#[derive(Debug, Deserialize)]
struct BarsResp { bars: Vec<DailyBar> }

#[derive(Debug, Clone)]
pub struct OptionChainRequest {
    pub underlying: String,
}

pub async fn get_option_chain(_req: OptionChainRequest) -> Result<()> {
    Err(OrcastError::NotImplemented("option data REST client"))
}

pub async fn get_daily_bars(client: &reqwest::Client, req: &DailyBarsRequest, api_key: &str, api_secret: &str) -> Result<Vec<DailyBar>> {
    let url = format!(
        "{}/v2/stocks/{}/bars?timeframe=1Day&start={}&end={}",
        req.data_base_url.trim_end_matches('/'),
        req.symbol,
        req.start,
        req.end
    );
    let resp = client
        .get(url)
        .header("APCA-API-KEY-ID", api_key)
        .header("APCA-API-SECRET-KEY", api_secret)
        .send()
        .await
        .map_err(OrcastError::Http)?
        .error_for_status()
        .map_err(OrcastError::Http)?
        .json::<BarsResp>()
        .await
        .map_err(OrcastError::Http)?;
    Ok(resp.bars)
}


