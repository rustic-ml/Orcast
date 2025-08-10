### Environment
- Precedence (first non-empty is used)
  1) `PAPER_*`
  2) `APCA_*`

- Primary (paper-friendly)
  - `PAPER_apiKey` (required)
  - `PAPER_apiSecret` (required)
  - `PAPER_URL` default `https://paper-api.alpaca.markets`
- Fallbacks (also supported)
  - `APCA_API_KEY_ID`, `APCA_API_SECRET_KEY`
  - `APCA_TRADING_BASE_URL` default `https://paper-api.alpaca.markets`
- Market data & misc
  - `APCA_DATA_BASE_URL` default `https://data.alpaca.markets`
  - `APCA_OPTION_FEED` one of `indicative`, `opra`
  - `ORCAST_TIMEOUT_MS` default `10000`
  - `ORCAST_MAX_RETRIES` default `3`

Example `.env` (do not commit)
```
PAPER_apiKey=...
PAPER_apiSecret=...
PAPER_URL=https://paper-api.alpaca.markets
APCA_DATA_BASE_URL=https://data.alpaca.markets
APCA_OPTION_FEED=indicative
```

