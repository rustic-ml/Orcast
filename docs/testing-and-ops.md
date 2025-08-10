### Testing
- Unit: mock HTTP (e.g., `httpmock`) for REST; fixture snapshots
- Integration: paper keys via CI secrets; dry-run mode; assert order lifecycle
- Load: burst GETs within rate limits; verify backoff

Coverage
- Target ≥ 80% line coverage across `src/`
- Enforce in CI (e.g., `cargo tarpaulin --out Xml --line` with threshold)

### Observability
- Structured logs with PII redaction (never log secrets)
- Hooks for metrics (latency, error rate, rate-limit hits)

### Deployment
- Config via env; paper by default
- Graceful shutdown of streams; resume on reconnect

