use orcast::config::TradingConfig;
use orcast::http::build_http_client;
use orcast::market_data::{DailyBarsRequest, get_daily_bars, DailyBar};
use orcast::screener::{rank_universe, StrategyCategory};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = TradingConfig::load_from_env().expect("env not configured");
    let http = build_http_client()?;
    let data_base_url = std::env::var("APCA_DATA_BASE_URL").unwrap_or_else(|_| "https://data.alpaca.markets".to_string());

    let symbols = vec!["AAPL", "MSFT", "NVDA", "SPY", "QQQ"];
    let mut universe: Vec<(&str, Vec<DailyBar>)> = Vec::new();
    for sym in &symbols {
        let bars = get_daily_bars(
            &http,
            &DailyBarsRequest{
                symbol: sym.to_string(),
                start: "2025-01-01".into(),
                end: "2025-12-31".into(),
                data_base_url: data_base_url.clone(),
            },
            &cfg.api_key_id,
            &cfg.api_secret_key,
        ).await.unwrap_or_default();
        universe.push((sym, bars));
    }

    let top = rank_universe(&universe, StrategyCategory::LongCall, 3);
    println!("Top long-call tickers:");
    for t in top { println!("{} -> {:.4}", t.symbol, t.score); }

    Ok(())
}


