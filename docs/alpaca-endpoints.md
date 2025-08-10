### Authentication
- Headers: `APCA-API-KEY-ID`, `APCA-API-SECRET-KEY`
- Base URLs: live `https://api.alpaca.markets`, paper `https://paper-api.alpaca.markets`

### Trading API (REST)
- Orders
  - Create: POST `/v2/orders`
  - Get/List/Replace/Cancel: `/v2/orders/...`
  - Multi-leg: submit parent with `legs` (see SDK docs)
- Positions
  - List/open/close: `/v2/positions` family
  - Exercise options position: see “Exercise an Options Position” in Trading API reference
- Assets
  - Get option contracts: Trading API “Get Option Contracts” (Assets section)

Docs
- Trading API reference: [link](https://alpaca.markets/docs/api-references/trading-api/)
- Orders: [link](https://alpaca.markets/docs/api-references/trading-api/orders/)

### Market Data API (Options)
- Option chain snapshots: GET `https://data.alpaca.markets/v1beta1/options/snapshots/{UNDERLYING}`
- Latest quote/trade: see Option endpoints in Market Data reference

Docs
- Option chain: [link](https://docs.alpaca.markets/reference/optionchain)
- Real-time option data (WebSocket, msgpack): [link](https://docs.alpaca.markets/docs/real-time-option-data)

### Streaming
- Trading updates: WebSocket `wss://paper-api.alpaca.markets/stream` (or live equivalent)
- Options data: WebSocket `wss://stream.data.alpaca.markets/v1beta1/{indicative|opra}` (msgpack only)

