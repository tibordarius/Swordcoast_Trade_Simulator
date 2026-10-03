# PH4-009 Multi-Leg Physical Logistics

## Objective
Add deterministic physical shipments over a multi-leg route graph with finite shared route capacity and explicit in-transit cargo ownership.

## Scope
- commodity transport profiles for authoritative mass/volume;
- runtime logistics routes;
- deterministic shortest-travel-time pathfinding;
- open/closed route state;
- multi-leg itineraries;
- shipment-owned cargo inventory accounts;
- shared mass/volume capacity reservation per active leg;
- WaitingForCapacity and InTransit shipment states;
- scheduled leg arrival events;
- deterministic wake-up of waiting shipments when capacity is released/reopened;
- final physical delivery through EconomicTransaction;
- replay/snapshot coverage.

## Non-goals
- freight prices;
- vessels/fleet allocation;
- piracy/weather hazards;
- insurance;
- route congestion affecting travel time;
- transshipment handling cost/time;
- re-routing an already-dispatched itinerary;
- merchant AI.

## Invariants
- dispatch quantity is positive;
- source and destination are distinct markets/accounts;
- cargo mass/volume derives from registered commodity profile;
- itinerary uses only open routes whose per-shipment capacity can fit the cargo;
- cargo leaves source inventory at dispatch and stays in a shipment-owned Holding account until final delivery;
- destination inventory changes only after final leg arrival;
- active route reservations never exceed mass/volume capacity;
- waiting order is deterministic;
- route closure prevents new leg departure but does not teleport or cancel in-transit cargo;
- failed dispatch/arrival processing is atomic;
- replay/snapshot produce identical shipment/capacity state.
