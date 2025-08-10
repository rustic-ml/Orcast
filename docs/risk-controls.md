### Pre-trade checks
- Account approvals level permits requested action (L1/L2/L3)
- Time-in-force and cutoff windows
  - On expiration days, option orders must be submitted before 3:15 p.m. ET
  - For broad-based ETFs (e.g., SPY, QQQ), cutoff is 3:30 p.m. ET
- Buying power and max order notional/qty
- Liquidity sanity: spread width, min quote size, open interest (from contract metadata)

### Safeguards
- Kill-switch env flag; per-underlying trade disable list
- Price collars for marketable orders
- Idempotent client order IDs for safe retries

