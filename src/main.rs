use orcast::{greet, config::TradingConfig};

fn main() {
    // Load config from env and print a minimal confirmation
    match TradingConfig::load_from_env() {
        Ok(cfg) => {
            println!("{}", greet("World"));
            println!(
                "Trading configured for: {} (key: {}…)",
                cfg.trading_base_url,
                &cfg.api_key_id.chars().take(4).collect::<String>()
            );
        }
        Err(err) => {
            eprintln!("Failed to load trading config: {err}");
            std::process::exit(1);
        }
    }
}
