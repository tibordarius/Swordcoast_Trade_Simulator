# [[Sprint 18 — DM Control Room]]

## Status

**Reference-complete contract + static control-room prototype.**

## World-event contract

The DM may inject causes such as:

- route disruption;
- production shock;
- demand shock;
- tax change;
- weather;
- military requisition;
- port disruption.

Supported modifiers affect systems such as route capacity, route risk, transport cost, production, consumption, taxes, port throughput and information delay.

## Hard guardrail

**Direct price overrides are forbidden.**

The DM cannot inject `WDEX Grain = 5.0 cp`.

Instead the DM injects something like:

```text
Pirate blockade
→ route capacity -60%
→ route risk +22%
→ shipments delayed/reduced
→ inventories fall
→ price model responds
```

## [[Why Did This Move?]]

Price explanations retain deterministic components and evidence links. Example:

```text
Reserve pressure       +8000 bps
Incoming supply loss   +3500 bps → SHIP-77
Route risk             +1200 bps → EV-1 blockade
Speculation             +500 bps
```

Evidence can be tagged public, rumor-level or DM-only so market information itself can become gameplay.
