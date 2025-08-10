use orcast::config::TradingConfig;
use orcast::alpaca::AlpacaTradingClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load env vars (PAPER_* preferred)
    let cfg = TradingConfig::load_from_env().expect("env not configured");
    let client = AlpacaTradingClient::new(&cfg).expect("client build failed");

    // Build a simple market order (single-leg). Note: this only builds the request for demonstration.
    let body = serde_json::json!({
        "symbol": "SPY240920C00450000", // example option symbol (not sent)
        "qty": "1",
        "side": "buy",
        "type": "market",
        "time_in_force": "day"
    });

    let req = client.build_create_order_request(body)?;
    println!("POST {}", req.url());
    println!("Headers: {:?}", req.headers());
    Ok(())
}


