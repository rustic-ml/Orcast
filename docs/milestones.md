### Milestones
- M0 Scaffold
  - Authenticated client; typed errors; CI build
- M1 Options data (REST)
  - Chain snapshot; latest quote/trade; minimal caching
- M2 Single-leg trading
  - Submit/replace/cancel; trade updates stream; positions open/close
- M3 Multi-leg trading
  - Build legs; submit/cancel parent; exercise endpoint
- M4 Risk & ops
  - Pre-trade checks; config; logs/metrics hooks

Each milestone is done when example flows in `examples/simple.rs` succeed against paper.

