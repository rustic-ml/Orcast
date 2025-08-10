use crate::config::TradingConfig;
use reqwest::{header::HeaderMap, header::HeaderValue, Method, Request, Url};

#[derive(Debug, thiserror::Error)]
pub enum AlpacaBuildError {
    #[error("failed to build HTTP client: {0}")]
    ClientBuild(reqwest::Error),
    #[error("invalid url: {0}")]
    InvalidUrl(String),
    #[error("failed to build request: {0}")]
    RequestBuild(reqwest::Error),
}

#[derive(Clone)]
pub struct AlpacaTradingClient {
    base_url: String,
    #[allow(dead_code)]
    api_key_id: String,
    #[allow(dead_code)]
    api_secret_key: String,
    http: reqwest::Client,
}

impl AlpacaTradingClient {
    pub fn new(config: &TradingConfig) -> Result<Self, AlpacaBuildError> {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            "APCA-API-KEY-ID",
            HeaderValue::from_str(&config.api_key_id).map_err(|_| AlpacaBuildError::InvalidUrl("api key header".into()))?,
        );
        default_headers.insert(
            "APCA-API-SECRET-KEY",
            HeaderValue::from_str(&config.api_secret_key).map_err(|_| AlpacaBuildError::InvalidUrl("api secret header".into()))?,
        );

        let http = reqwest::Client::builder()
            .default_headers(default_headers)
            .build()
            .map_err(AlpacaBuildError::ClientBuild)?;

        Ok(Self {
            base_url: config.trading_base_url.clone(),
            api_key_id: config.api_key_id.clone(),
            api_secret_key: config.api_secret_key.clone(),
            http,
        })
    }

    pub fn build_create_order_request(&self, body_json: serde_json::Value) -> Result<Request, AlpacaBuildError> {
        let url = format!("{}/v2/orders", self.base_url);
        let url = normalize_url(&url).ok_or_else(|| AlpacaBuildError::InvalidUrl(url.clone()))?;
        self.http
            .request(Method::POST, url)
            .header("Content-Type", "application/json")
            .body(body_json.to_string())
            .build()
            .map_err(AlpacaBuildError::RequestBuild)
    }

    pub fn build_get_positions_request(&self) -> Result<Request, AlpacaBuildError> {
        let url = format!("{}/v2/positions", self.base_url);
        let url = normalize_url(&url).ok_or_else(|| AlpacaBuildError::InvalidUrl(url.clone()))?;
        self.http
            .request(Method::GET, url)
            .build()
            .map_err(AlpacaBuildError::RequestBuild)
    }
}

fn normalize_url(s: &str) -> Option<Url> {
    let trimmed = s.trim_end_matches('/');
    Url::parse(trimmed).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::TradingConfig;

    fn test_config() -> TradingConfig {
        TradingConfig {
            trading_base_url: "https://paper-api.alpaca.markets".to_string(),
            api_key_id: "KEY123".to_string(),
            api_secret_key: "SECRET456".to_string(),
        }
    }

    #[test]
    fn builds_requests_with_headers() {
        let cfg = test_config();
        let client = AlpacaTradingClient::new(&cfg).unwrap();
        let body = serde_json::json!({"symbol":"SPY","qty":"1","side":"buy","type":"market","time_in_force":"day"});
        let req = client.build_create_order_request(body).unwrap();
        assert_eq!(req.url().as_str(), "https://paper-api.alpaca.markets/v2/orders");
        assert_eq!(req.method(), reqwest::Method::POST);
        // Content-Type header should be present for JSON body
        assert_eq!(req.headers().get("Content-Type").unwrap(), "application/json");
    }

    #[test]
    fn get_positions_url() {
        let cfg = test_config();
        let client = AlpacaTradingClient::new(&cfg).unwrap();
        let req = client.build_get_positions_request().unwrap();
        assert_eq!(req.url().as_str(), "https://paper-api.alpaca.markets/v2/positions");
    }
}


