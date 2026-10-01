use std::collections::BTreeSet;

use crate::constants::{OPENING_INVENTORY_ACCOUNT_ID, OPENING_MONEY_ACCOUNT_ID};
use crate::error::{ValidationError, ValidationIssue};
use crate::model::{ScenarioPack, SCENARIO_PACK_SCHEMA_VERSION};

fn duplicate_issues<'a>(
    collection: &str,
    ids: impl Iterator<Item = &'a str>,
    issues: &mut Vec<ValidationIssue>,
) {
    let mut seen = BTreeSet::new();
    for id in ids {
        if !seen.insert(id.to_owned()) {
            issues.push(ValidationIssue::new(
                format!("{collection}[id={id}]"),
                "duplicate_id",
                format!("duplicate stable ID {id}"),
            ));
        }
    }
}

fn require_name(path: &str, name: &str, issues: &mut Vec<ValidationIssue>) {
    if name.trim().is_empty() {
        issues.push(ValidationIssue::new(
            path,
            "empty_name",
            "name must not be empty",
        ));
    }
}

pub fn validate(pack: &ScenarioPack) -> Result<(), ValidationError> {
    let mut issues = Vec::new();

    if pack.manifest.schema_version != SCENARIO_PACK_SCHEMA_VERSION {
        issues.push(ValidationIssue::new(
            "manifest.schema_version",
            "unsupported_schema_version",
            format!(
                "expected schema version {}, found {}",
                SCENARIO_PACK_SCHEMA_VERSION, pack.manifest.schema_version
            ),
        ));
    }

    if pack.manifest.pack_id.as_str().trim().is_empty() {
        issues.push(ValidationIssue::new(
            "manifest.pack_id",
            "empty_pack_id",
            "pack_id must not be empty",
        ));
    }

    if pack.manifest.campaign_epoch.trim().is_empty() {
        issues.push(ValidationIssue::new(
            "manifest.campaign_epoch",
            "empty_campaign_epoch",
            "campaign_epoch must not be empty",
        ));
    }

    duplicate_issues(
        "units",
        pack.units.iter().map(|record| record.id.as_str()),
        &mut issues,
    );
    duplicate_issues(
        "commodities",
        pack.commodities.iter().map(|record| record.id.as_str()),
        &mut issues,
    );
    duplicate_issues(
        "places",
        pack.places.iter().map(|record| record.id.as_str()),
        &mut issues,
    );
    duplicate_issues(
        "markets",
        pack.markets.iter().map(|record| record.id.as_str()),
        &mut issues,
    );
    duplicate_issues(
        "routes",
        pack.routes.iter().map(|record| record.id.as_str()),
        &mut issues,
    );
    duplicate_issues(
        "inventory_accounts",
        pack.inventory_accounts.iter().map(|record| record.id.as_str()),
        &mut issues,
    );
    duplicate_issues(
        "money_accounts",
        pack.money_accounts.iter().map(|record| record.id.as_str()),
        &mut issues,
    );

    let unit_ids: BTreeSet<&str> = pack.units.iter().map(|record| record.id.as_str()).collect();
    let commodity_ids: BTreeSet<&str> = pack
        .commodities
        .iter()
        .map(|record| record.id.as_str())
        .collect();
    let place_ids: BTreeSet<&str> = pack.places.iter().map(|record| record.id.as_str()).collect();
    let market_ids: BTreeSet<&str> = pack
        .markets
        .iter()
        .map(|record| record.id.as_str())
        .collect();
    let inventory_account_ids: BTreeSet<&str> = pack
        .inventory_accounts
        .iter()
        .map(|record| record.id.as_str())
        .collect();
    let money_account_ids: BTreeSet<&str> = pack
        .money_accounts
        .iter()
        .map(|record| record.id.as_str())
        .collect();

    for unit in &pack.units {
        let path = format!("units[id={}]", unit.id);
        require_name(&format!("{path}.name"), &unit.name, &mut issues);

        if unit.dimension.trim().is_empty() {
            issues.push(ValidationIssue::new(
                format!("{path}.dimension"),
                "empty_dimension",
                "unit dimension must not be empty",
            ));
        }

        if unit.numerator == 0 || unit.denominator == 0 {
            issues.push(ValidationIssue::new(
                format!("{path}.conversion"),
                "invalid_conversion",
                "unit conversion numerator and denominator must both be non-zero",
            ));
        }

        if !unit_ids.contains(unit.base_unit.as_str()) {
            issues.push(ValidationIssue::new(
                format!("{path}.base_unit"),
                "unknown_unit",
                format!("unknown base unit {}", unit.base_unit),
            ));
        }
    }

    for commodity in &pack.commodities {
        let path = format!("commodities[id={}]", commodity.id);
        require_name(&format!("{path}.name"), &commodity.name, &mut issues);

        if !unit_ids.contains(commodity.base_unit.as_str()) {
            issues.push(ValidationIssue::new(
                format!("{path}.base_unit"),
                "unknown_unit",
                format!("unknown base unit {}", commodity.base_unit),
            ));
        }

        if commodity.mass_grams_per_base_unit == 0 || commodity.volume_cm3_per_base_unit == 0 {
            issues.push(ValidationIssue::new(
                format!("{path}.physical_dimensions"),
                "invalid_physical_dimensions",
                "active commodity mass and volume per base unit must both be greater than zero",
            ));
        }
    }

    for place in &pack.places {
        require_name(
            &format!("places[id={}].name", place.id),
            &place.name,
            &mut issues,
        );
    }

    for market in &pack.markets {
        let path = format!("markets[id={}]", market.id);
        require_name(&format!("{path}.name"), &market.name, &mut issues);

        if !place_ids.contains(market.place_id.as_str()) {
            issues.push(ValidationIssue::new(
                format!("{path}.place_id"),
                "unknown_place",
                format!("unknown place {}", market.place_id),
            ));
        }
    }

    for route in &pack.routes {
        let path = format!("routes[id={}]", route.id);

        if !market_ids.contains(route.from_market.as_str()) {
            issues.push(ValidationIssue::new(
                format!("{path}.from_market"),
                "unknown_market",
                format!("unknown market {}", route.from_market),
            ));
        }

        if !market_ids.contains(route.to_market.as_str()) {
            issues.push(ValidationIssue::new(
                format!("{path}.to_market"),
                "unknown_market",
                format!("unknown market {}", route.to_market),
            ));
        }

        if route.from_market == route.to_market {
            issues.push(ValidationIssue::new(
                path.clone(),
                "route_self_loop",
                "route endpoints must be distinct markets",
            ));
        }

        if route.distance_meters == 0 {
            issues.push(ValidationIssue::new(
                format!("{path}.distance_meters"),
                "invalid_distance",
                "route distance must be greater than zero",
            ));
        }

        if route.base_travel_ticks == sim_kernel_v2::SimTick::ZERO {
            issues.push(ValidationIssue::new(
                format!("{path}.base_travel_ticks"),
                "invalid_travel_time",
                "route base travel time must be greater than zero",
            ));
        }
    }

    for account in &pack.inventory_accounts {
        if account.id.as_str() == OPENING_INVENTORY_ACCOUNT_ID {
            issues.push(ValidationIssue::new(
                format!("inventory_accounts[id={}]", account.id),
                "reserved_id",
                "ID is reserved by ScenarioPack initialization",
            ));
        }
    }

    for account in &pack.money_accounts {
        if account.id.as_str() == OPENING_MONEY_ACCOUNT_ID {
            issues.push(ValidationIssue::new(
                format!("money_accounts[id={}]", account.id),
                "reserved_id",
                "ID is reserved by ScenarioPack initialization",
            ));
        }
    }

    let mut opening_inventory_keys = BTreeSet::new();
    for opening in &pack.opening_inventory {
        let key = format!("{}|{}", opening.account_id, opening.commodity_id);
        let path = format!(
            "opening_inventory[account={},commodity={}]",
            opening.account_id, opening.commodity_id
        );

        if !opening_inventory_keys.insert(key) {
            issues.push(ValidationIssue::new(
                path.clone(),
                "duplicate_opening_balance",
                "duplicate opening inventory row",
            ));
        }

        if !inventory_account_ids.contains(opening.account_id.as_str()) {
            issues.push(ValidationIssue::new(
                format!("{path}.account_id"),
                "unknown_inventory_account",
                format!("unknown inventory account {}", opening.account_id),
            ));
        }

        if !commodity_ids.contains(opening.commodity_id.as_str()) {
            issues.push(ValidationIssue::new(
                format!("{path}.commodity_id"),
                "unknown_commodity",
                format!("unknown commodity {}", opening.commodity_id),
            ));
        }

        if opening.quantity < 0 {
            issues.push(ValidationIssue::new(
                format!("{path}.quantity"),
                "negative_opening_balance",
                "opening inventory quantity must be non-negative",
            ));
        }
    }

    let mut opening_money_keys = BTreeSet::new();
    for opening in &pack.opening_money {
        let path = format!("opening_money[account={}]", opening.account_id);

        if !opening_money_keys.insert(opening.account_id.as_str().to_owned()) {
            issues.push(ValidationIssue::new(
                path.clone(),
                "duplicate_opening_balance",
                "duplicate opening money row",
            ));
        }

        if !money_account_ids.contains(opening.account_id.as_str()) {
            issues.push(ValidationIssue::new(
                format!("{path}.account_id"),
                "unknown_money_account",
                format!("unknown money account {}", opening.account_id),
            ));
        }

        if opening.amount_cp < 0 {
            issues.push(ValidationIssue::new(
                format!("{path}.amount_cp"),
                "negative_opening_balance",
                "opening money amount must be non-negative",
            ));
        }
    }

    issues.sort();

    if issues.is_empty() {
        Ok(())
    } else {
        Err(ValidationError { issues })
    }
}
