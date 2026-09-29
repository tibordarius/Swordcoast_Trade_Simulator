# Hand-Worked Grain Flow Example

**Purpose:** Sprint 0 acceptance test. The values below are calibration examples, not final economic lore.

## Scenario

An independent merchant notices that [[Athkatla]] has excess [[Grain]] while [[Waterdeep]] is below its desired reserve.

The test proves the full state transition:

`production -> inventory -> price -> purchase -> shipment -> arrival -> resale -> P&L`

## Commodity Definition

```yaml
grain:
  id: CMD-GRAIN
  base_unit: kg
  quantity_scale: 1000   # internal milli-kg
  reference_price_cp_per_kg: 2
  perishability: medium
  mass_grams_per_unit: 1000
```

For readability below, quantities are shown in kg and prices in cp/kg.

## Starting Market State

### [[Athkatla]]

- on hand: 1,000,000 kg
- reserved: 50,000 kg
- available: 950,000 kg
- target reserve: 700,000 kg
- normalized daily consumption: 25,000 kg
- available cover: 38 days
- fundamental price: 1.70 cp/kg
- ask before order: 1.75 cp/kg

### [[Waterdeep]]

- on hand: 720,000 kg
- reserved: 80,000 kg
- available: 640,000 kg
- target reserve: 1,200,000 kg
- normalized daily consumption: 45,000 kg
- available cover: 14.2 days
- fundamental price: 2.55 cp/kg
- expected bid if conditions persist: 2.48 cp/kg

## Merchant

```yaml
capital_cp: 500000
risk_tolerance: 0.50
information_delay_days:
  Athkatla: 0
  Waterdeep: 4
```

The merchant sees a stale but attractive Waterdeep bid and evaluates a shipment of 100,000 kg.

## Step 1 — Opportunity Calculation

Purchase estimate:

`100,000 kg × 1.75 cp = 175,000 cp`

Estimated transport/tariffs/storage:

`32,000 cp`

Expected loss allowance:

`8,000 cp`

Expected destination revenue using observed price:

`100,000 × 2.48 cp = 248,000 cp`

Expected profit:

`248,000 - 175,000 - 32,000 - 8,000 = 33,000 cp`

Expected ROI on deployed capital:

`33,000 / 207,000 = 15.94%`

Merchant accepts.

## Step 2 — Market Purchase

The merchant submits a 100,000 kg market buy.

Because the purchase is large relative to immediate Athkatlan sell liquidity, execution slips:

| Quantity | Execution price |
|---:|---:|
| 30,000 kg | 1.75 cp |
| 40,000 kg | 1.79 cp |
| 30,000 kg | 1.84 cp |

Total purchase cost:

`52,500 + 71,600 + 55,200 = 179,300 cp`

Average execution price:

`1.793 cp/kg`

Athkatla inventory immediately changes:

- on hand remains physically present until loading;
- reserved increases by 100,000 kg;
- available decreases by 100,000 kg;
- market pressure rises slightly.

## Step 3 — Shipment Departure

When loading completes:

Athkatla:

- on hand decreases by 100,000 kg;
- reserved decreases by 100,000 kg.

Shipment created:

```yaml
commodity: CMD-GRAIN
quantity_kg: 100000
origin: Athkatla
destination: Waterdeep
owner: independent_merchant_001
departure_tick: 100000
eta_tick: 105184
status: in_transit
```

The example ETA represents 18 days at 288 five-minute ticks/day. Final route time remains to be calibrated.

Waterdeep does **not** receive inventory yet.

If Waterdeep knows the shipment is coming, `incoming_committed` may increase, discounted by ETA/reliability when forming expectations.

## Step 4 — Transit

No per-tick position writes occur.

Visual position is derived from:

`progress = (current_tick - departure_tick) / (eta_tick - departure_tick)`

A route-risk event may destroy, delay or divert the cargo. In this example, no loss occurs.

## Step 5 — Arrival

At tick 105184:

- shipment status -> `arrived`;
- Waterdeep on-hand grain +100,000 kg;
- Waterdeep incoming committed -100,000 kg;
- provenance records Athkatla as commercial origin and the upstream production node when available.

The added stock reduces scarcity pressure before the merchant necessarily sells it if the market models visible warehouse inventory.

## Step 6 — Destination Sale

During transit, Waterdeep's shortage worsened slightly, but other merchants also reacted.

Current executable bid ladder:

| Quantity | Bid |
|---:|---:|
| 25,000 kg | 2.62 cp |
| 35,000 kg | 2.57 cp |
| 40,000 kg | 2.49 cp |

Revenue:

`65,500 + 89,950 + 99,600 = 255,050 cp`

## Step 7 — Realized P&L

- sale revenue: 255,050 cp
- purchase cost: 179,300 cp
- actual transport/tariff/storage: 31,500 cp
- cargo loss: 0

Realized profit:

`255,050 - 179,300 - 31,500 = 44,250 cp`

Realized ROI:

`44,250 / 210,800 = 20.99%`

The realized result differs from the estimate because both origin slippage and destination market conditions changed.

## State/Accounting Invariants Checked

- [x] grain produced somewhere before it can be sold;
- [x] purchase reserves physical inventory;
- [x] departure removes it from origin;
- [x] transit does not duplicate it;
- [x] arrival adds it exactly once;
- [x] money is deducted/credited consistently;
- [x] route costs reduce P&L;
- [x] large orders experience slippage;
- [x] destination price can change while cargo is at sea;
- [x] position is derived, not continuously written;
- [x] the trade remains understandable from causal state.

## What This Example Reveals We Still Need To Specify

1. Market clearing/liquidity curve.
2. Whether physical ownership changes at order execution or loading.
3. How warehouse-visible inventory affects local price before sale.
4. How incoming shipments are known to different actors.
5. How risk events are scheduled and resolved.
6. How transport costs are split between charter, crew, port fees, insurance and tariffs.
7. How perishability affects grain during long voyages.

These are the first questions Sprint 1–4 implementation should answer, in that order.
