use orcast::alpaca::AlpacaTradingClient;
use orcast::config::TradingConfig;

#[test]
fn builds_urls_against_base() {
    let cfg = TradingConfig {
        trading_base_url: "https://paper-api.alpaca.markets".to_string(),
        api_key_id: "KEY123".to_string(),
        api_secret_key: "SECRET456".to_string(),
    };
    let client = AlpacaTradingClient::new(&cfg).unwrap();
    let req = client.build_get_positions_request().unwrap();
    assert!(req.url().as_str().ends_with("/v2/positions"));
}


