### Orcast: OHLCV‑Driven Options Ticker Screener (Rust)

Purpose: suggest tickers for options strategies using daily OHLCV features.

- Key capabilities
  - Option chains and latest quotes/trades (REST; streaming optional)
  - Single-/multi-leg order build; positions; trade-updates stream (scaffolded)
  - Risk gates and execution tuning hooks
  - Strategy modules per options strategy

See `docs/GETTING_STARTED.md` for a fast start.

Config, endpoints, tuning, risk, and test notes are in `docs/REFERENCE.md`.

### Endpoints & config
See `docs/REFERENCE.md`.

### Module layout
- `config`: env loading with precedence and safe test resolver
- `http`: shared reqwest client
- `market_data`: daily OHLCV fetch (stocks)
- `screener`: rank tickers per strategy category from OHLCV
- `streaming`: trade updates and option quotes (stubs)
- `strategies`: per‑strategy modules emitting legs
  - `long_call`, `long_put`, `covered_call`, `protective_put`,
    `bull_call_spread`, `bear_put_spread`, `long_straddle`, `long_strangle`

### Examples (runnable)
- `simple.rs`: config sanity
- `screen_tickers.rs`: score a universe from daily OHLCV and suggest top tickers

### Risk controls (must pass before send)
- Approvals level (L1/L2/L3) and buying power
- Time windows, incl. expiration‑day cutoffs (3:15 p.m. ET; 3:30 p.m. ET broad ETFs)
- Liquidity checks: max spread vs mid, min size, quote freshness
- Kill switch and per‑symbol disable; price collars for marketable orders

### Execution tuning
See `docs/REFERENCE.md`.

### Testing & coverage
See `docs/REFERENCE.md`.

### Milestones (summary)
- M0: scaffold; M1: data; M2: single‑leg; M3: multi‑leg; M4: risk/ops

### Quickstart (paper)
- Export env
  - `export PAPER_URL=https://paper-api.alpaca.markets/`
  - `export PAPER_apiKey=...`
  - `export PAPER_apiSecret=...`
- Run
  - `cargo run`

