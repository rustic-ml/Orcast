### Orcast: Alpaca Options Trading (Rust)

Purpose: lean Rust client for Alpaca US equity options. Paper first; Level 3 (multi‑leg) ready.

- Key capabilities
  - Option chains and latest quotes/trades (REST; streaming optional)
  - Single-/multi-leg order build; positions; trade-updates stream (scaffolded)
  - Risk gates and execution tuning hooks
  - Strategy modules per options strategy

### Quickstart (paper)
- Export
  - `export PAPER_URL=https://paper-api.alpaca.markets/`
  - `export PAPER_apiKey=...`
  - `export PAPER_apiSecret=...`
- Run: `cargo run`
- Examples: `cargo run --example place_single_leg_order`

### Config (env precedence)
1) `PAPER_*`: `PAPER_apiKey`, `PAPER_apiSecret`, `PAPER_URL` (default paper URL)
2) `APCA_*`: `APCA_API_KEY_ID`, `APCA_API_SECRET_KEY`, `APCA_TRADING_BASE_URL`
- Optional: `APCA_DATA_BASE_URL`, `APCA_OPTION_FEED` (`indicative`|`opra`)

### Endpoints (reference)
- Trading (REST base: paper `https://paper-api.alpaca.markets`)
  - Orders: POST/GET/PATCH/DELETE `/v2/orders`
  - Positions: `/v2/positions` (+ exercise options)
  - Option contracts (Assets): "Get Option Contracts"
- Market data (Options)
  - Chain snapshots: GET `https://data.alpaca.markets/v1beta1/options/snapshots/{UNDERLYING}`
  - Real-time (WebSocket msgpack): `wss://stream.data.alpaca.markets/v1beta1/{indicative|opra}`

### Module layout
- `config`: env loading with precedence and safe test resolver
- `http`: shared reqwest client
- `alpaca`: request builders for orders/positions
- `trading`: single-leg submit scaffold; map strategy legs → order JSON
- `market_data`: option chain/quotes (stub)
- `streaming`: trade updates and option quotes (stubs)
- `strategies`: per‑strategy modules emitting legs
  - `long_call`, `long_put`, `covered_call`, `protective_put`,
    `bull_call_spread`, `bear_put_spread`, `long_straddle`, `long_strangle`

### Examples (runnable)
- `simple.rs`: config sanity
- `place_single_leg_order.rs`: single‑leg order body → POST request
- `place_multi_leg_order.rs`: multi‑leg body → POST request
- `get_positions.rs`: GET positions request
- `stream_trade_updates.rs`: streaming placeholder

### Risk controls (must pass before send)
- Approvals level (L1/L2/L3) and buying power
- Time windows, incl. expiration‑day cutoffs (3:15 p.m. ET; 3:30 p.m. ET broad ETFs)
- Liquidity checks: max spread vs mid, min size, quote freshness
- Kill switch and per‑symbol disable; price collars for marketable orders

### Execution tuning
- Feed: prefer `opra` if entitled; otherwise `indicative`
- Use streaming for quotes/trades; avoid polling
- Price near mid with collars; multi‑leg priced as net debit/credit
- Retries: idempotent client_order_id, bounded backoff; no retries past cutoff
- Respect rate limits; cache chains (~60s); reuse HTTP client
- Track time‑to‑fill, slippage vs mid, rejects → tighten gates iteratively

### Testing & coverage
- Unit: config, request builders, strategy legs
- Integration: paper dry‑runs, httpmock stubs
- Load: burst GETs within rate limits
- Coverage goal: ≥ 80% lines; enforce in CI

### Milestones (summary)
- M0: scaffold (auth client, errors)
- M1: options data (REST)
- M2: single‑leg trading + trade updates stream
- M3: multi‑leg trading + exercise
- M4: risk & ops (gates, metrics)

### Quickstart (paper)
- Export env
  - `export PAPER_URL=https://paper-api.alpaca.markets/`
  - `export PAPER_apiKey=...`
  - `export PAPER_apiSecret=...`
- Run
  - `cargo run`

