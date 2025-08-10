### Orcast: Alpaca Options Trading (Rust)

Purpose: a lean Rust service/library to trade US equity options via Alpaca’s Trading API. Paper first, then live. Supports single-leg and multi-leg (Level 3) strategies.

- Key capabilities
  - Fetch option chains and quotes
  - Place/track single- and multi-leg orders
  - Stream order updates; optional real-time option data
  - Basic risk checks and position management

- Read next
  - System design: `docs/system-design.md`
  - Implementation plan: `docs/implementation-plan.md`
  - Alpaca endpoints: `docs/alpaca-endpoints.md`
  - Config: `docs/config.md`
  - Milestones: `docs/milestones.md`
  - Risk controls: `docs/risk-controls.md`
  - Testing & ops: `docs/testing-and-ops.md`

### Quickstart (paper)
- Export env
  - `export PAPER_URL=https://paper-api.alpaca.markets/`
  - `export PAPER_apiKey=...`
  - `export PAPER_apiSecret=...`
- Run
  - `cargo run`

