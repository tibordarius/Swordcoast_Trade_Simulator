# PH4-009 Acceptance

- [ ] Commodity transport profiles validate positive mass/volume.
- [ ] Logistics routes validate endpoints, travel time, and capacity.
- [ ] Deterministic pathfinder finds the lowest-travel-time open route and stable tie-break.
- [ ] Dispatch rejects missing profile/path/accounts and non-positive quantity atomically.
- [ ] Cargo moves into a shipment-owned Holding account at dispatch.
- [ ] Destination inventory remains unchanged until final arrival.
- [ ] Multi-leg shipment advances one scheduled leg at a time.
- [ ] Shared route reservations never exceed mass/volume capacity.
- [ ] Full route capacity causes deterministic WaitingForCapacity state.
- [ ] Capacity release wakes waiting shipments deterministically.
- [ ] Closing a future leg strands shipment at transfer rather than failing world time advancement.
- [ ] Reopening a route resumes eligible waiting shipments.
- [ ] Final arrival settles cargo through EconomicTransaction.
- [ ] Replay/snapshot preserve itineraries, reservations, waiting state, and deliveries.
- [ ] Snapshot schema advances for logistics state/payloads.
- [ ] Existing information/market/production/ScenarioPack tests remain green.
- [ ] Full GitHub CI passes.
