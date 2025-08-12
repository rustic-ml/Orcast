### Orcast improvements to boost options-screener ticker efficiency

This document proposes focused enhancements in the Orcast library to improve ticker shortlisting efficiency and quality for different options strategy categories consumed by `options-screener`.

### Baseline (current)
- Uses OHLCV-only heuristics: momentum, ATR-based range, average volume.
- Scores per `StrategyCategory` and returns top N.
- Requires ≥20 daily bars.

### High‑impact, provider‑agnostic enhancements
- **Liquidity gates**
  - **min per-strike open interest** (e.g., ≥ 250 OI for ATM ±3 strikes)
  - **min option volume** (daily) and **max bid–ask spread** (e.g., ≤ 25–50 bps)
  - **min chain depth** around ATM (e.g., ±5 strikes, ≥ 2 expiries)

- **Volatility analytics**
  - **IV Rank/Percentile** across multiple windows (3m, 6m, 12m)
  - **IV/Realized-vol ratio** and **realized vol (ATR, stdev of returns)**
  - **Term structure slope** (near vs next expiry), **25-delta skew** (calls/puts)

- **Event awareness**
  - **Days to earnings/FOMC** windows; configurable avoidance or preference
  - **Historical post-event move** stats and **IV crush** tendency

- **Unusual options activity**
  - **Unusual volume vs OI** and vs 30d median
  - Large block/flow proxy flags (if data available)

- **Regime and quality filters**
  - **Beta**, correlation to benchmark/sector ETF; avoid micro-caps
  - **Price floor** (e.g., ≥ $5) to reduce spread/fill issues

### Category‑specific scoring upgrades
- **Long Call** (directional up)
  - Emphasize trend strength (RS vs sector/ETF, ADX), price above MAs
  - Prefer moderate IV rank (e.g., 30–60) + tight spreads + adequate OI
  - De‑weight near-earnings unless explicitly event-driven

- **Long Put** (directional down)
  - Favor downtrend strength (negative RS, MA alignment, breakdowns)
  - Allow higher IV rank if trend quality is strong; maintain liquidity gates

- **Long Straddle** (long vol)
  - Prefer high IV percentile and/or rising IV into catalysts
  - Require high realized range and robust liquidity near ATM
  - Incorporate earnings proximity and historical IV crush to balance risk

- **Covered Call** (income, neutral-to-bullish, lower vol)
  - Favor lower realized vol, stable names with **elevated IV rank**
  - Screen for strong liquidity, modest drawdown profile, optional dividend awareness

### Data and API additions (Orcast)
- Add an `OptionFeatures` struct with fields such as:
  - `iv_rank_3m`, `iv_rank_6m`, `iv_rank_1y`, `iv_percentile_*`
  - `iv_to_realized_ratio`, `term_structure_slope`, `call_put_skew`
  - `min_oi_atm_window`, `opt_volume_atm_window`, `max_spread_bps_atm`
  - `days_to_earnings`, `historical_post_earn_move`, `iv_crush_risk`
  - `unusual_activity_score`
- Provide pluggable providers (chains/IV, earnings calendar). Keep OHLCV-only fallback.

### CLI extensions (options-screener)
- Add arguments to control new gates/weights:
  - `--min-oi`, `--min-opt-volume`, `--max-spread-bps`
  - `--min-iv-rank`, `--max-iv-rank`
  - `--max-days-to-earnings`, `--allow-earnings`
  - `--min-price`, `--max-beta`
  - `--liquidity-hard-gate` (drop tickers failing gates before scoring)

### Scoring framework changes
- Introduce per-category weighted scoring using the new features.
- Keep hard gates for liquidity and event constraints to reduce noise.
- Emit explanation fields in results (e.g., "IV Rank 72; Spread 14 bps; OI OK; Earnings in 9d").

### Phased implementation plan
- **Phase 1 (Low risk, high ROI)**: Liquidity gates + IV Rank/Percentile + CLI flags.
- **Phase 2**: Event awareness (earnings proximity, IV crush), per-category weights.
- **Phase 3**: Term structure and skew; regime filters (beta/correlation).
- **Phase 4 (Optional)**: Unusual activity + simple ML ranking with cross-validation.

### Success metrics
- Higher proportion of shortlisted tickers with tight spreads and adequate OI
- Reduced median spread (bps) and improved fillability proxies
- Better realized performance in category-aligned backtests (vol capture for straddles, income yield for covered calls, trend continuation for calls/puts)


