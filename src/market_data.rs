use crate::error::{OrcastError, Result};
use serde::Deserialize;
use std::collections::HashMap;

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

pub fn read_daily_bars_from_csv<R: std::io::Read>(reader: R) -> Result<Vec<DailyBar>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let headers = rdr
        .headers()
        .map_err(|e| OrcastError::Config(format!("csv headers: {e}")))?
        .clone();
    let index: HashMap<String, usize> = headers
        .iter()
        .enumerate()
        .map(|(i, h)| (h.trim().to_ascii_lowercase(), i))
        .collect();

    let mut rows = Vec::new();
    for rec in rdr.records() {
        let rec = rec.map_err(|e| OrcastError::Config(format!("csv record: {e}")))?;
        let get = |key: &str| -> Result<&str> {
            let k = key.to_ascii_lowercase();
            let pos = index
                .get(&k)
                .ok_or_else(|| OrcastError::Config(format!("missing column: {key}")))?;
            rec.get(*pos)
                .ok_or_else(|| OrcastError::Config(format!("column out of bounds: {key}")))
        };

        let open: f64 = get("open")?.parse().map_err(|e| OrcastError::Config(format!("open parse: {e}")))?;
        let high: f64 = get("high")?.parse().map_err(|e| OrcastError::Config(format!("high parse: {e}")))?;
        let low: f64 = get("low")?.parse().map_err(|e| OrcastError::Config(format!("low parse: {e}")))?;
        let close: f64 = get("close")?.parse().map_err(|e| OrcastError::Config(format!("close parse: {e}")))?;
        let timestamp = get("timestamp")?.to_string();
        let volume: f64 = get("volume")?.parse().map_err(|e| OrcastError::Config(format!("volume parse: {e}")))?;

        rows.push(DailyBar { t: timestamp, o: open, h: high, l: low, c: close, v: volume });
    }
    Ok(rows)
}


