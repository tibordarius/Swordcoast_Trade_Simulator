# HTTP API v1

## Read endpoints

### `GET /api/v1/worlds/{world_id}/branches/{branch_id}/snapshot`

Returns the current coherent client snapshot and sequence watermark.

### `GET /api/v1/markets`

Filters: `world_id`, `branch_id`, optional `exchange_code`.

### `GET /api/v1/markets/{market_id}/commodities`

Returns current price/inventory state for the selected market.

### `GET /api/v1/shipments`

Filters include `status`, `owner_trader_id`, `commodity_id`, `origin_market_id`, `destination_market_id`.

### `GET /api/v1/routes`

Returns current route state plus stable geometry references.

### `GET /api/v1/deltas?after_sequence={sequence}`

Bounded reconnect/catch-up endpoint.

## Command endpoint

### `POST /api/v1/commands`

All state mutation enters through commands. The HTTP handler never edits simulation state directly.

Request:

```json
{
  "world_id":"1",
  "branch_id":"1",
  "command_id":"client-generated-id",
  "expected_sequence":"8425",
  "type":"place_market_order",
  "payload":{}
}
```

Response is an acknowledgement, not necessarily the resulting state:

```json
{
  "accepted":true,
  "command_id":"client-generated-id",
  "input_event_sequence":"912"
}
```

The authoritative consequence returns through committed deltas.

## Optimistic concurrency

Mutation commands carry `expected_sequence`. Commands whose assumptions require a current market state may be rejected when the branch has moved too far ahead.

## Error shape

```json
{
  "code":"stale_market_state",
  "message":"Market state changed after the order preview.",
  "current_sequence":"8431"
}
```
