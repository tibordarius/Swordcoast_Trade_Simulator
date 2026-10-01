# PH4-006 Handoff

## Goal
Implement the first scheduled physical production and population-consumption loop.

## Status
IN_PROGRESS

## Branch
`phase4/v2-production-consumption-impl`

## Supersedes
The earlier `phase4/v2-production-consumption` branch was created before PH4-005 merged. Its reviewed ID and scheduler-payload work was carried forward onto this clean branch; the stale branch should not be resumed.

## Completed
- typed production/population IDs;
- EventDomain::Population;
- typed EventPayload;
- scheduler pop-next-due and record-fired primitives.

## Next action
Add production and population domain state types.

## Do not redo
Do not implement pricing or market demand here. Consumption removes physical goods and records served/unmet quantities only.
