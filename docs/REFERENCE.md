### Reference

Config precedence
- `PAPER_*` → `APCA_*`; optional `APCA_DATA_BASE_URL` (default `https://data.alpaca.markets`), `APCA_OPTION_FEED`

Endpoints
- Trading base: paper `https://paper-api.alpaca.markets`
  - Orders: `/v2/orders`
  - Positions: `/v2/positions` (exercise options supported)
  - Option contracts: Trading API Assets → Get Option Contracts
- Market data
  - Daily OHLCV (stocks): `v2/stocks/{symbol}/bars` (TimeFrame=1Day)
  - Option chain snapshots: `v1beta1/options/snapshots/{UNDERLYING}`
  - WebSocket: `v1beta1/{indicative|opra}` (msgpack)

Execution tuning
- Liquidity gates: spread ≤ X% mid; min size; fresh quotes
- Pricing: limit near mid; multi‑leg priced as net debit/credit
- Retries: idempotent client_order_id; bounded backoff; no retries past cutoff
- Rate/perf: reuse HTTP client; cache chains (~60s); back off on 429

Risk
- Approvals level, buying power, time windows (3:15 ET; 3:30 ET broad ETFs)
- Kill switch; per‑symbol disable; collars for marketable orders

Testing
- Unit: config, builders, strategies
- Screener: deterministic scores for fixtures; mock data client
- Integration: paper dry‑run; httpmock stubs
- Coverage: ≥ 80%


