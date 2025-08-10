use crate::market_data::DailyBar;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StrategyCategory {
    LongCall,
    LongPut,
    LongStraddle,
    CoveredCall,
}

#[derive(Debug, Clone)]
pub struct ScoredTicker {
    pub symbol: String,
    pub score: f64,
}

// Simple heuristics from OHLCV: momentum, volatility, and range features
pub fn score_ticker(symbol: &str, bars: &[DailyBar], cat: StrategyCategory) -> Option<ScoredTicker> {
    if bars.len() < 20 { return None; }
    let closes: Vec<f64> = bars.iter().map(|b| b.c).collect();
    let vols: Vec<f64> = bars.iter().map(|b| b.v).collect();
    let last = *closes.last()?;
    let first = closes[0];
    let ret = (last - first) / first; // simple momentum over window
    let avg_true_range = bars.iter().map(|b| b.h - b.l).sum::<f64>() / bars.len() as f64;
    let avg_vol = vols.iter().sum::<f64>() / vols.len() as f64;

    let score = match cat {
        StrategyCategory::LongCall => 0.7 * ret + 0.3 * (avg_true_range / last),
        StrategyCategory::LongPut => 0.7 * (-ret) + 0.3 * (avg_true_range / last),
        StrategyCategory::LongStraddle => avg_true_range / last,
        StrategyCategory::CoveredCall => 0.6 * (avg_vol / 1_000_000.0) - 0.2 * (avg_true_range / last) + 0.6 * (ret.max(0.0)),
    };

    Some(ScoredTicker { symbol: symbol.to_string(), score })
}

pub fn rank_universe<'a>(universe: impl IntoIterator<Item=&'a (&'a str, Vec<DailyBar>)>, cat: StrategyCategory, top_n: usize) -> Vec<ScoredTicker> {
    let mut scored: Vec<ScoredTicker> = universe
        .into_iter()
        .filter_map(|(sym, bars)| score_ticker(sym, bars, cat.clone()))
        .collect();
    scored.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    scored.truncate(top_n);
    scored
}


