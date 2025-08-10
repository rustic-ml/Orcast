### Modules
- config: loads env, validates base/data URLs, feed choice
- http: shared async client (reqwest), retry with jitter, 429 backoff, observability hooks
- alpaca::trading: accounts, orders (single/multi-leg), positions, exercise, trade updates stream client
- alpaca::options_data: option chain snapshots, latest quote/trade (REST); optional streaming client (msgpack)
- risk: pre-trade checks (account approvals, TIF windows, buying power, liquidity)
- strategy: trait-based strategies operating on market + account state
- runner: orchestrates ticks/event-loop, handles graceful shutdown
- storage (optional): JSON/SQLite for run artifacts and trade logs

### Error handling
- Map HTTP errors with typed context; retry idempotent GETs, not POSTs (except safe network failures before send).
- Rate-limit: respect headers; exponential backoff with cap.

### Performance
- Cache option chains per underlying per minute; reuse connections; batch symbol queries where supported.

