# Sword Coast Trade Simulator

A deterministic economic and logistics simulator for the Forgotten Realms, focused initially on the Sword Coast.

The project models settlements, production, consumption, inventories, merchant behaviour, trade routes, physical shipments, market prices, historical analytics, DM-created world events, and exchange-style interfaces such as the Waterdeep Exchange (WDEX).

## Current status

The independent reference implementation is green through Sprint 20, with Sprint 21 providing release/preflight gates. The native stack is designed around:

- Rust for the deterministic simulation core
- PostgreSQL + PostGIS for durable world and geographic state
- DuckDB + Parquet for historical analytics
- JavaScript browser clients for WDEX, DM controls, and live logistics
- Toril GIS as the canonical geographic coordinate/data source

The current milestone is to validate the Rust, PostGIS, and DuckDB implementation in GitHub Actions, then connect the live API to the browser clients.

## Repository layout

- `code/` — Rust workspace, SQL migrations, browser clients, contracts, analytics
- `seed/` — initial markets, commodities, routes, production chains, merchant profiles
- `reference/` — deterministic reference models and expected outputs
- `tools/` — validators, release checks, GIS import/build tools
- `00_HOME.md` … `41_SPRINT_21_RELEASE_PREP.md` — project design, roadmap, decisions, research, sprint records
- `.github/workflows/ci.yml` — native release gates

## Core rule

Prices are not random numbers. Shocks may be random, but prices emerge from physical production, inventories, demand, market depth, logistics, risk, and trader behaviour.

## Geography

Canonical world coordinates use the Toril GIS coordinate system. The first exchange network covers Calimport, Athkatla, Baldur's Gate, Waterdeep, Neverwinter, and Luskan.

## License / setting note

This is an unofficial fan project for a private tabletop campaign and is not endorsed by Wizards of the Coast.
