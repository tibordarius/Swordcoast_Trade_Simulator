# PH4-009 Plan

1. Add logistics domain types:
   - CommodityTransportProfile;
   - LogisticsRoute;
   - Itinerary / ItineraryLeg;
   - Shipment / ShipmentStatus;
   - LogisticsState.
2. Implement deterministic pathfinding over open routes.
3. Add registration commands for transport profiles/routes.
4. Add route-open/closed command.
5. Add DispatchShipment command.
6. Generate one private cargo Holding account per shipment.
7. Move cargo source → shipment account through EconomicTransaction.
8. Reserve the first route leg when capacity exists, otherwise wait.
9. Add typed ShipmentLegArrival scheduler payload.
10. On arrival, release capacity, advance to next leg, or settle final cargo delivery.
11. Wake waiting shipments deterministically after capacity release/route reopen.
12. Add integration tests for multi-leg delivery, finite capacity queues, closure/reopen, atomic failure, pathfinding, replay, and snapshots.
13. Bump snapshot schema and run full CI.

Compatibility:
- market/information truth remains unchanged;
- no freight pricing or merchant behavior;
- v1 untouched.
