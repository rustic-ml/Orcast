use orcast::config::TradingConfig;
use orcast::alpaca::AlpacaTradingClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = TradingConfig::load_from_env().expect("env not configured");
    let client = AlpacaTradingClient::new(&cfg).expect("client build failed");

    // Multi-leg order body (example long straddle), this demonstrates structure; not sent.
    let body = serde_json::json!({
        "symbol": "SPY",               // underlying for grouping
        "order_class": "simple",        // parent container; legs define contracts
        "legs": [
            {
                "symbol": "SPY240920C00450000",
                "qty": "1",
                "side": "buy",
                "type": "limit",
                "limit_price": "2.50",
                "time_in_force": "day"
            },
            {
                "symbol": "SPY240920P00450000",
                "qty": "1",
                "side": "buy",
                "type": "limit",
                "limit_price": "2.40",
                "time_in_force": "day"
            }
        ]
    });

    let req = client.build_create_order_request(body)?;
    println!("POST {}", req.url());
    println!("Headers: {:?}", req.headers());
    Ok(())
}


