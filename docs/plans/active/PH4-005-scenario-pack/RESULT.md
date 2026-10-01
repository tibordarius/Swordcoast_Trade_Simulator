# PH4-005 Result

## Outcome
Minimal executable ScenarioPack v2 boundary implemented.

## Added
- standalone scenario-pack-v2 crate;
- strict JSON loader and deterministic validation;
- provenance status classification;
- typed units, commodities, places, markets, routes, accounts, and opening balances;
- immutable ScenarioRegistry in sim-kernel-v2;
- deterministic pack-to-registry compilation;
- deterministic opening-state command compilation;
- opening stock and money initialized through balanced EconomicTransaction postings;
- 3-market / 5-commodity tiny Sword Coast fixture;
- scenario-pack-validate CLI;
- cargo xtask pack-validate;
- CI gate exercising the actual validator command.

## Verification
GitHub Actions run 36878536869 passed all jobs.

## Architectural result
The engine now has a strict external-data boundary. TorilGIS and later migration tooling can target ScenarioPack v2 without gaining direct mutation access to the kernel.

## Follow-up
PH4-006 should add the first physical economy loop: minimal production batches plus population-cohort consumption, both posting through the ledger and scheduled-event architecture.
