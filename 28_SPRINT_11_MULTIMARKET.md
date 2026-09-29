# Sprint 11 — Multi-Market Network

## Status

**Six-exchange grain network and executable spread engine implemented at reference level.**

## Exchanges

- [[CALEX]] — [[Calimport]]
- [[ATHEX]] — [[Athkatla]]
- [[BGEX]] — [[Baldur's Gate]]
- [[WDEX]] — [[Waterdeep]]
- [[NWEX]] — [[Neverwinter]]
- [[LUSEX]] — [[Luskan]]

## Delivered

- `code/web/wdex/mock-network.json`
- `code/web/wdex/network.mjs`
- `code/web/wdex/network.test.mjs`

The network reference scenario is explicitly `simulation_generated`; its calibration values are tests, not Forgotten Realms canon.

## Spread calculation

The system does not compare `origin ask < destination bid` and call that profit. For a requested cargo size it now calculates:

1. executable purchase after origin slippage;
2. an available route/path with enough capacity;
3. freight over the chosen path;
4. compounded expected route loss;
5. executable destination sale after destination slippage;
6. required capital, expected profit and ROI.

Path finding can choose a multi-hop route when that is cheaper on a risk-adjusted per-unit basis.

## Capacity lesson retained

A large headline spread is not tradeable when no available route can carry the requested lot. The reference test deliberately asks for an oversized cargo and receives zero opportunities.

## Exit direction

The next geographic sprint should replace simulation-generated route calibration with measured/curated Sword Coast route distances, travel times, seasonal capacity and risk profiles while preserving provenance.
