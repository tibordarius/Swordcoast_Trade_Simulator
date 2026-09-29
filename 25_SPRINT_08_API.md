# Sprint 8 — API & Live State

## Status

**Protocol contract and client replay semantics are reference-tested. Axum implementation remains pending native Rust compilation.**

## Delivered

- `code/contracts/live-protocol.md`
- `code/contracts/http-api.md`
- `reference/sprint8_live_protocol.mjs`

## Decisions

### JSON numeric safety

All authoritative 64-bit IDs, ticks, sequence numbers, money and commodity quantities cross JSON as decimal strings. Clients parse them as `BigInt` or preserve them as strings. This prevents silent precision loss above JavaScript's safe integer range.

### Snapshot + delta protocol

A client starts from a coherent snapshot with a sequence watermark, then applies monotonically ordered committed deltas. Duplicate batches are ignored; gaps are detected and trigger catch-up or snapshot replacement.

### Command boundary

Clients send commands, not direct state mutations. The API acknowledges command acceptance; committed outcomes arrive through the same deterministic delta stream as every other state change.

### Ship positions

Live ship positions are never streamed every frame. The client derives position from route geometry, departure tick, ETA and current simulation tick.

## Reference result

`SPRINT8_LIVE_PROTOCOL: PASS`

The test also includes an inventory value above JavaScript's `Number.MAX_SAFE_INTEGER` to prove the transport encoding remains exact.

## Next implementation cards

- Axum read routes;
- command endpoint;
- branch delta buffer;
- WebSocket subscription;
- snapshot-required fallback;
- authentication boundary before multi-user deployment.
