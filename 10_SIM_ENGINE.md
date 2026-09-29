# Simulation Engine Specification

## Primary Rule

The simulation should be **causal, deterministic and inspectable**.

A price move must ultimately be explainable through changes in inventory, production, consumption, expectations, risk, logistics or orders.

## Authoritative Time

Recommended baseline:

- authoritative engine tick can represent a small fixed interval such as 1–5 in-world minutes;
- subsystems do not all execute every tick;
- scheduled events are keyed to future ticks;
- UI animation is independent from simulation frequency.

The exact base tick should be benchmarked before being frozen.

## Tick Phases

```text
1. Apply scheduled world/input events
2. Apply production due this tick
3. Apply consumption due this tick
4. Update known inventory commitments
5. Process market orders / clear markets
6. Calculate prices and explanation components
7. Run merchant decision agents due this tick
8. Create/cancel shipments
9. Resolve shipment arrivals/loss events due this tick
10. Deterministically reduce deltas
11. Commit state
12. Publish state delta
13. Periodically checkpoint/history flush
```

Not every phase must do work every base tick.

## Deterministic Parallelism

Safe pattern:

```text
immutable old state
      |
parallel calculations
      |
private delta buffers
      |
stable sort / deterministic reduction
      |
authoritative commit
```

Never allow arbitrary concurrent mutation of authoritative market state.

## Pricing v0

A simple first model:

```text
reference_price
× scarcity_factor
× demand_factor
× risk_factor
× bounded_expectation_factor
```

### Scarcity

Function of:

- on-hand stock;
- reserved stock;
- target reserve;
- expected near-term arrivals.

### Demand

Function of:

- baseline sector consumption;
- unmet demand backlog;
- temporary event demand;
- production-input demand.

### Risk

Function of expected replacement/transport risk, route disruption, taxes and supply reliability.

### Expectations / momentum

Small bounded component based on observable recent trends. It must not overwhelm fundamentals in v0.

## Market Depth

Do not create thousands of fake NPC limit orders. Represent aggregated liquidity mathematically or with a small synthetic ladder derived from available inventory, willingness to sell/buy and local market depth.

Large player/merchant orders consume liquidity and produce slippage.

## Merchant AI v1

Each decision cycle:

1. observe accessible markets;
2. estimate buy cost;
3. estimate destination sale price;
4. calculate transport/tax/storage cost;
5. estimate expected risk loss;
6. calculate expected profit and ROI;
7. apply capital, capacity and strategy constraints;
8. select opportunity;
9. place order / create shipment.

```text
expected_profit =
expected_revenue
- purchase_cost
- transport_cost
- taxes
- storage
- insurance_or_expected_loss
```

## Information Latency

Do not assume every actor sees every market instantly.

Each trader can have:

- information delay by market;
- information confidence;
- private information sources;
- faction communication bonuses.

This permits [[Whale Society]], [[OOS]], [[OTC]] and [[DTC/DWTC (Deepwater Trading Company)]] to differ without cheating the price engine.

## Shipment Model

Authoritative shipment state:

- route;
- departure;
- ETA;
- current status;
- risk events already resolved/queued.

Visual position is derived from progress along geometry.

## World Events

Events modify causes rather than outputs.

Good:

`Nelanther piracy risk +40%`

`Amnian grain production -25%`

`Waterdeep naval demand +300 grain/day`

Bad:

`Waterdeep grain price +30%`

## Provenance of Economic Effects

Each major price calculation should expose a compact decomposition so the application can answer:

> Why did this move?

Potential components:

- inventory deficit;
- consumption spike;
- production reduction;
- delayed arrivals;
- route risk;
- tariffs;
- speculative order flow;
- substitution effects.

## Conservation Invariants

At minimum test:

- goods are not created except by production/event rules;
- goods are not destroyed except by consumption/spoilage/loss/event rules;
- money changes owners consistently;
- one shipment cannot arrive twice;
- reserved inventory cannot be double-sold;
- market execution cannot exceed available liquidity;
- branch state cannot mutate its parent.
