# ADR-0002: TorilGIS / ScenarioPack Boundary

Status: Accepted

## Context
TorilGIS is strongest at source identity, provenance, geography, registry review, and campaign authoring. The simulation kernel needs a stable executable input rather than direct access to source files.

## Decision
TorilGIS remains the world-data and authoring platform. v2 consumes a versioned immutable ScenarioPack.

## Consequences
- Source/canon review remains outside the kernel.
- Scenario assumptions are explicit.
- Runtime code does not crawl Google Drive, Obsidian, or TorilGIS internals.
- Stable IDs connect simulation results back to GIS/lore/provenance.
