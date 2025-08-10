### Scope
- Options market data (chains, snapshots, latest quotes/trades) and order execution (single, multi-leg), positions, exercise, order/position streaming.
- Start with paper trading; swap to live via config only.

### Non-goals (initial)
- Portfolio analytics/backtesting, GUI, user auth flows, brokerage account onboarding.

### Phases (deliverables are acceptance criteria)
1) M0: Project scaffold
   - Reqwest client with auth, retries, rate-limit backoff
   - Basic types: order, option contract/chain
2) M1: Market data (REST)
   - Option chain snapshots for an underlying
   - Latest quote/trade for a contract
3) M2: Single-leg orders
   - Submit/replace/cancel; fetch positions; close positions
   - Trade updates stream subscription
4) M3: Multi-leg (Level 3)
   - Build legs; submit multi-leg; cancel/replace parent
   - Position lifecycle (liquidate, exercise)
5) M4: Risk & ops
   - Pre-trade checks (level, buying power, liquidity windows)
   - Logging, redaction, metrics hooks

### Implementation notes
- Keep APIs async; isolate Alpaca specifics behind adapters.
- Prefer snapshots/REST initially; enable option data streaming as opt-in.

### Examples coverage (external interfaces)
- Provide runnable examples in `examples/` for:
  - Config loading: `simple.rs`
  - Single-leg order build: `place_single_leg_order.rs`
  - Multi-leg order build: `place_multi_leg_order.rs`
  - Positions retrieval: `get_positions.rs`
  - Trade updates streaming placeholder: `stream_trade_updates.rs` (to be implemented by M2/M3)

### Quality gates
- Test coverage target: ≥ 80% lines for `src/` via unit + integration tests
- CI to fail if coverage < target

