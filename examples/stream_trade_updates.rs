// Placeholder showcasing where trade updates streaming would be implemented.
// In the minimal scaffold we only print config to confirm environment.
use orcast::config::TradingConfig;

fn main() {
    let cfg = TradingConfig::load_from_env().expect("env not configured");
    println!("Configured for {} with key {}…", cfg.trading_base_url, &cfg.api_key_id[..4]);
    println!("Streaming client not yet implemented in this scaffold.");
}


