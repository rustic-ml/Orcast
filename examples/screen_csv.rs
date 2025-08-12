use orcast::market_data::read_daily_bars_from_csv;
use orcast::screener::{rank_universe, StrategyCategory};
use orcast::util::prepare_result_dir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Usage: cargo run --example screen_csv -- path/to/AAPL.csv path/to/MSFT.csv
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: screen_csv <csv1> <csv2> ...");
        std::process::exit(1);
    }
    // Clean results dir before run
    prepare_result_dir("result")?;
    let mut universe = Vec::new();
    for path in &args[1..] {
        let sym = std::path::Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("TICKER");
        let f = std::fs::File::open(path)?;
        let bars = read_daily_bars_from_csv(f)?;
        universe.push((sym, bars));
    }
    let top = rank_universe(&universe, StrategyCategory::LongCall, 5);
    println!("Top tickers (LongCall):");
    for t in top { println!("{} -> {:.4}", t.symbol, t.score); }
    Ok(())
}


