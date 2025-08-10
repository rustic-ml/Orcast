use orcast::config::TradingConfig;
use orcast::alpaca::AlpacaTradingClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = TradingConfig::load_from_env().expect("env not configured");
    let client = AlpacaTradingClient::new(&cfg).expect("client build failed");

    let req = client.build_get_positions_request()?;
    println!("GET {}", req.url());
    println!("Headers: {:?}", req.headers());
    Ok(())
}


