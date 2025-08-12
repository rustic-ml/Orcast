use crate::market_data::DailyBar;
use crate::features::{OptionDataProvider, OptionFeatures};

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
    pub explain: Option<String>,
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

    Some(ScoredTicker { symbol: symbol.to_string(), score, explain: None })
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

pub fn rank_universe_with_features<'a, P: OptionDataProvider>(
    universe: impl IntoIterator<Item=&'a (&'a str, Vec<DailyBar>)>,
    cat: StrategyCategory,
    top_n: usize,
    provider: &P,
) -> Vec<ScoredTicker> {
    let mut scored: Vec<ScoredTicker> = Vec::new();
    for (sym, bars) in universe.into_iter() {
        if let Some(mut s) = score_ticker(sym, bars, cat.clone()) {
            let f: OptionFeatures = provider.fetch_features(sym);
            // Liquidity hard gates (if provided)
            let liquid_ok = f.min_oi_atm_window.unwrap_or(1) >= 1
                && f.max_spread_bps_atm.unwrap_or(10.0) <= 1000.0;
            if !liquid_ok { continue; }

            // Simple IV rank weight (if present)
            if let Some(iv_rank) = f.iv_rank_6m.or(f.iv_rank_3m).or(f.iv_rank_1y) {
                match cat {
                    StrategyCategory::LongStraddle => { s.score += 0.3 * (iv_rank / 100.0); }
                    StrategyCategory::CoveredCall => { s.score += 0.2 * (iv_rank / 100.0); }
                    _ => {}
                }
            }
            s.explain = Some(format!("iv_rank6m={:?} min_oi_atm={:?} max_spread_bps_atm={:?}", f.iv_rank_6m, f.min_oi_atm_window, f.max_spread_bps_atm));
            scored.push(s);
        }
    }
    scored.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    scored.truncate(top_n);
    scored
}


