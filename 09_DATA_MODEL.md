# Data Model

## Core Entity Graph

```text
Region
  -> Settlement
      -> Market
      -> Warehouse
      -> Consumption Sector
      -> Production Site

Commodity
  -> Recipe Input/Output
  -> Market Inventory
  -> Price Series
  -> Shipment Batch

Route Node
  -> Route Edge
      -> Shipment

Trader / Faction
  -> Capital
  -> Warehouse
  -> Orders
  -> Shipments

World
  -> Branch
      -> Tick
      -> Event Log
      -> Snapshot
```

## `world`

- id
- name
- seed
- simulation_version
- current_tick
- current_datetime
- branch_id
- status

## `branch`

- id
- world_id
- parent_branch_id
- parent_snapshot_id
- name
- created_at
- canonical boolean

## `settlement`

- id
- canonical_name
- aliases
- settlement_type
- region_id
- location geometry
- population_reference
- port_capability
- river_capability
- road_capability
- provenance_id

## `commodity`

- id
- name
- family
- base_unit
- fixed_point_scale
- mass_per_unit
- volume_per_unit
- perishability_class
- legality_class
- reference_price
- provenance_id

## `market`

- id
- settlement_id
- market_type
- liquidity_class
- tax_profile_id

## `market_inventory`

- market_id
- commodity_id
- on_hand
- reserved
- target_reserve
- incoming_known
- outgoing_committed
- updated_tick

## `market_state`

- market_id
- commodity_id
- bid
- ask
- mid
- last
- volatility
- pressure_supply
- pressure_demand
- pressure_risk
- pressure_momentum
- updated_tick

## `production_site`

- id
- settlement_id/region_id
- site_type
- owner_id
- capacity
- season_profile
- provenance_id

## `recipe`

- id
- name
- cycle_time
- facility_type

## `recipe_input`

- recipe_id
- commodity_id
- quantity

## `recipe_output`

- recipe_id
- commodity_id
- quantity

## `consumption_sector`

- id
- settlement_id
- type
- scale
- demand_profile

Examples:

- households_low_income;
- households_artisan;
- nobility;
- taverns;
- military;
- shipyards;
- construction;
- arcane institutions.

## `route_node`

- id
- geometry
- node_type

## `route_edge`

- id
- from_node
- to_node
- route_type
- geometry
- distance
- base_time
- capacity
- monetary_cost
- base_risk
- seasonal_profile
- active

## `trader`

- id
- name
- trader_type
- faction_id
- capital
- risk_tolerance
- information_quality
- strategy_profile

## `shipment`

- id
- owner_id
- origin_market_id
- destination_market_id
- route_id
- commodity_id
- quantity
- purchase_value
- transport_cost
- departure_tick
- eta_tick
- status
- batch_id

## `batch`

- id
- commodity_id
- origin_site_id
- created_tick
- provenance_chain
- quality

Use batches only where provenance matters. Bulk interchangeable goods may aggregate batches after rules permit it.

## `order`

- id
- actor_id
- market_id
- commodity_id
- side
- order_type
- quantity
- remaining_quantity
- limit_price
- created_tick
- status

## `trade_execution`

- id
- order_id
- market_id
- commodity_id
- quantity
- price
- buyer_id
- seller_id
- tick

## `world_event`

- id
- branch_id
- event_type
- source
- start_tick
- end_tick
- geographic_scope
- entity_scope
- modifiers
- visibility

## `input_event_log`

Append-only external inputs:

- sequence
- branch_id
- tick_received
- actor
- command_type
- payload
- schema_version
- hash

## `snapshot`

- id
- branch_id
- tick
- simulation_version
- state_uri
- checksum
- created_at

## `provenance`

- id
- classification: `fr_canon | campaign_canon | inference | simulation_generated`
- source_title
- source_reference
- page_or_location
- note
- confidence

## Time-Series Fact

Recent market observations:

- branch_id
- tick
- market_id
- commodity_id
- open
- high
- low
- close
- volume
- inventory
- imports
- exports

Older observations migrate to Parquet.
