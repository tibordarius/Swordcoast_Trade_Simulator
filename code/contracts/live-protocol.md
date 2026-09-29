# Live Protocol v1

## Purpose

Provide a deterministic, reconnectable API boundary between the authoritative simulation writer and clients such as WDEX, the DM control room and the map.

## Numeric encoding

Authoritative 64-bit values are encoded as **base-10 JSON strings**:

- IDs;
- simulation ticks;
- event/delta sequence numbers;
- `*_mcp` money values;
- `*_milli` commodity quantities;
- large distances/capacities.

Small bounded values such as basis points, liquidity tier and percentages may remain JSON numbers.

This avoids JavaScript `Number` precision loss while keeping the protocol inspectable.

## Snapshot envelope

```json
{
  "protocol_version": 1,
  "world_id": "1",
  "branch_id": "1",
  "simulation_version": "0.1.0",
  "sequence": "8422",
  "tick": "550230",
  "markets": [],
  "shipments": []
}
```

`sequence` is the last committed external/public delta included in the snapshot.

## Delta batch

The server may group many simulation changes into one network frame.

```json
{
  "protocol_version": 1,
  "world_id": "1",
  "branch_id": "1",
  "from_sequence": "8423",
  "to_sequence": "8425",
  "tick": "550235",
  "deltas": [
    {
      "sequence": "8423",
      "type": "market_state_changed",
      "key": {"market_id":"4","commodity_id":"1"},
      "data": {"bid_mcp":"2110","ask_mcp":"2136"}
    }
  ]
}
```

## Ordering

- sequence is strictly increasing per branch;
- deltas are committed before publication;
- clients apply deltas only in sequence order;
- duplicate sequence numbers are idempotently ignored;
- a gap triggers catch-up or snapshot recovery.

## Reconnect

Client reconnects with its last applied sequence:

`/api/v1/live?world_id=1&branch_id=1&after_sequence=8425`

Server responses:

1. replay retained deltas if available;
2. otherwise return `snapshot_required` and the current snapshot endpoint.

## Initial delta types

- `market_inventory_changed`
- `market_state_changed`
- `shipment_created`
- `shipment_status_changed`
- `route_state_changed`
- `trader_account_changed`
- `world_event_started`
- `world_event_ended`
- `simulation_clock_changed`

Do not emit visual ship positions. Position is derived client-side from route geometry, departure tick, ETA and current tick.

## Backpressure

The API should prioritize latest coherent state over preserving every visual update. Authoritative history remains in the event/history stores. If a slow client exceeds the retained delta window, require a new snapshot.
