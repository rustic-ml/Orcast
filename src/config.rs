use std::env;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TradingConfig {
    pub trading_base_url: String,
    pub api_key_id: String,
    pub api_secret_key: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    Missing(String),
}

impl TradingConfig {
    pub fn load_from_env() -> Result<Self, ConfigError> {
        let trading_base_url = read_env_multi(&["PAPER_URL", "APCA_TRADING_BASE_URL"]) // prefer explicit paper URL
            .unwrap_or_else(|| "https://paper-api.alpaca.markets".to_string());

        let api_key_id = read_env_multi(&["PAPER_apiKey", "APCA_API_KEY_ID"]) 
            .ok_or_else(|| ConfigError::Missing("PAPER_apiKey or APCA_API_KEY_ID".into()))?;

        let api_secret_key = read_env_multi(&["PAPER_apiSecret", "APCA_API_SECRET_KEY"]) 
            .ok_or_else(|| ConfigError::Missing("PAPER_apiSecret or APCA_API_SECRET_KEY".into()))?;

        Ok(TradingConfig {
            trading_base_url: trading_base_url.trim_end_matches('/').to_string(),
            api_key_id,
            api_secret_key,
        })
    }

    #[cfg(test)]
    pub fn load_with_resolver<F>(mut resolve: F) -> Result<Self, ConfigError>
    where
        F: FnMut(&str) -> Option<String>,
    {
        let trading_base_url = read_env_multi_with(&["PAPER_URL", "APCA_TRADING_BASE_URL"], &mut resolve)
            .unwrap_or_else(|| "https://paper-api.alpaca.markets".to_string());

        let api_key_id = read_env_multi_with(&["PAPER_apiKey", "APCA_API_KEY_ID"], &mut resolve)
            .ok_or_else(|| ConfigError::Missing("PAPER_apiKey or APCA_API_KEY_ID".into()))?;

        let api_secret_key = read_env_multi_with(&["PAPER_apiSecret", "APCA_API_SECRET_KEY"], &mut resolve)
            .ok_or_else(|| ConfigError::Missing("PAPER_apiSecret or APCA_API_SECRET_KEY".into()))?;

        Ok(TradingConfig {
            trading_base_url: trading_base_url.trim_end_matches('/').to_string(),
            api_key_id,
            api_secret_key,
        })
    }
}

fn read_env_multi(candidates: &[&str]) -> Option<String> {
    for key in candidates {
        if let Ok(value) = env::var(key) {
            if !value.is_empty() {
                return Some(value);
            }
        }
    }
    None
}

#[cfg(test)]
fn read_env_multi_with<F>(candidates: &[&str], resolve: &mut F) -> Option<String>
where
    F: FnMut(&str) -> Option<String>,
{
    for key in candidates {
        if let Some(value) = resolve(key) {
            if !value.is_empty() {
                return Some(value);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_from_paper_vars() {
        let mut map = std::collections::HashMap::new();
        map.insert("PAPER_URL".to_string(), "https://paper-api.alpaca.markets/".to_string());
        map.insert("PAPER_apiKey".to_string(), "KEY123".to_string());
        map.insert("PAPER_apiSecret".to_string(), "SECRET456".to_string());

        let cfg = TradingConfig::load_with_resolver(|k| map.get(k).cloned())
            .expect("expected config to load");
        assert_eq!(cfg.trading_base_url, "https://paper-api.alpaca.markets");
        assert_eq!(cfg.api_key_id, "KEY123");
        assert_eq!(cfg.api_secret_key, "SECRET456");
    }
}

