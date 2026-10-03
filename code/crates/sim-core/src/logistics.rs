use crate::InventoryLedger;
use serde::{Deserialize, Serialize};

pub const MILLI_BASE_UNIT: i64 = 1_000;
pub const PROGRESS_BPS_MAX: i64 = 10_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Route {
    pub travel_ticks: u64,
    pub capacity_per_day: i64,
    pub variable_cost_mcp_per_base_unit: i64,
    pub fixed_cost_mcp: i64,
}

impl Route {
    pub fn transport_cost_mcp(&self, quantity_milli_base_units: i64) -> i64 {
        assert!(quantity_milli_base_units >= 0);
        self.fixed_cost_mcp
            + (quantity_milli_base_units * self.variable_cost_mcp_per_base_unit) / MILLI_BASE_UNIT
    }

    pub fn accepts(&self, quantity_milli_base_units: i64) -> bool {
        quantity_milli_base_units > 0 && quantity_milli_base_units <= self.capacity_per_day
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShipmentStatus {
    Reserved,
    InTransit,
    Arrived,
    Lost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Shipment {
    pub id: u64,
    pub quantity: i64,
    pub departure_tick: u64,
    pub eta_tick: u64,
    pub status: ShipmentStatus,
}

impl Shipment {
    pub fn new(id: u64, quantity: i64, departure_tick: u64, eta_tick: u64) -> Self {
        assert!(quantity > 0);
        assert!(eta_tick >= departure_tick);
        Self {
            id,
            quantity,
            departure_tick,
            eta_tick,
            status: ShipmentStatus::Reserved,
        }
    }

    pub fn progress_bps(&self, current_tick: u64) -> i64 {
        if current_tick <= self.departure_tick {
            return 0;
        }
        if current_tick >= self.eta_tick || self.eta_tick == self.departure_tick {
            return PROGRESS_BPS_MAX;
        }
        (((current_tick - self.departure_tick) as u128 * PROGRESS_BPS_MAX as u128)
            / (self.eta_tick - self.departure_tick) as u128) as i64
    }

    pub fn depart(&mut self, origin: &mut InventoryLedger) -> bool {
        if self.status != ShipmentStatus::Reserved || !origin.depart_reserved(self.quantity) {
            return false;
        }
        self.status = ShipmentStatus::InTransit;
        true
    }

    pub fn arrive(&mut self, destination: &mut InventoryLedger, current_tick: u64) -> bool {
        if self.status == ShipmentStatus::Arrived {
            return false;
        }
        if self.status != ShipmentStatus::InTransit || current_tick < self.eta_tick {
            return false;
        }
        if !destination.receive(self.quantity) {
            return false;
        }
        self.status = ShipmentStatus::Arrived;
        true
    }

    pub fn mark_lost(&mut self) -> bool {
        match self.status {
            ShipmentStatus::Reserved | ShipmentStatus::InTransit => {
                self.status = ShipmentStatus::Lost;
                true
            }
            ShipmentStatus::Arrived | ShipmentStatus::Lost => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CargoProfile {
    pub mass_grams_per_base_unit: i64,
    pub volume_cm3_per_base_unit: i64,
}

impl CargoProfile {
    pub fn new(mass_grams_per_base_unit: i64, volume_cm3_per_base_unit: i64) -> Self {
        assert!(mass_grams_per_base_unit > 0);
        assert!(volume_cm3_per_base_unit > 0);
        Self {
            mass_grams_per_base_unit,
            volume_cm3_per_base_unit,
        }
    }

    pub fn usage(&self, quantity_milli: i64) -> CargoUsage {
        assert!(quantity_milli > 0);
        let mass_grams = div_floor_i128(
            i128::from(quantity_milli) * i128::from(self.mass_grams_per_base_unit),
            1_000,
        );
        let volume_cm3 = div_floor_i128(
            i128::from(quantity_milli) * i128::from(self.volume_cm3_per_base_unit),
            1_000,
        );
        CargoUsage {
            mass_grams,
            volume_cm3,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CargoUsage {
    pub mass_grams: i64,
    pub volume_cm3: i64,
}

impl CargoUsage {
    pub fn checked_add(self, other: Self) -> Self {
        Self {
            mass_grams: self
                .mass_grams
                .checked_add(other.mass_grams)
                .expect("cargo mass overflow"),
            volume_cm3: self
                .volume_cm3
                .checked_add(other.volume_cm3)
                .expect("cargo volume overflow"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalibratedRoute {
    pub id: String,
    pub from_market: String,
    pub to_market: String,
    pub bidirectional: bool,
    pub travel_ticks: u64,
    pub risk_bps: i64,
    pub freight_mcp_per_kg: i64,
    pub capacity_kg_per_day: i64,
    pub capacity_m3_per_day: i64,
}

impl CalibratedRoute {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: impl Into<String>,
        from_market: impl Into<String>,
        to_market: impl Into<String>,
        bidirectional: bool,
        travel_ticks: u64,
        risk_bps: i64,
        freight_mcp_per_kg: i64,
        capacity_kg_per_day: i64,
        capacity_m3_per_day: i64,
    ) -> Self {
        assert!(travel_ticks > 0);
        assert!((0..=10_000).contains(&risk_bps));
        assert!(freight_mcp_per_kg >= 0);
        assert!(capacity_kg_per_day > 0);
        assert!(capacity_m3_per_day > 0);
        Self {
            id: id.into(),
            from_market: from_market.into(),
            to_market: to_market.into(),
            bidirectional,
            travel_ticks,
            risk_bps,
            freight_mcp_per_kg,
            capacity_kg_per_day,
            capacity_m3_per_day,
        }
    }

    pub fn supports_direction(&self, origin: &str, destination: &str) -> bool {
        (origin == self.from_market && destination == self.to_market)
            || (self.bidirectional && origin == self.to_market && destination == self.from_market)
    }

    pub fn freight_cost_mcp(&self, quantity_milli: i64, cargo: CargoProfile) -> i64 {
        assert!(quantity_milli > 0);
        let numerator = i128::from(quantity_milli)
            * i128::from(cargo.mass_grams_per_base_unit)
            * i128::from(self.freight_mcp_per_kg);
        div_round_i128(numerator, 1_000_000)
    }

    pub fn accepts_usage(&self, usage: CargoUsage) -> bool {
        usage.mass_grams > 0
            && usage.volume_cm3 > 0
            && i128::from(usage.mass_grams) <= i128::from(self.capacity_kg_per_day) * 1_000
            && i128::from(usage.volume_cm3) <= i128::from(self.capacity_m3_per_day) * 1_000_000
    }

    pub fn accepts_quantity(&self, quantity_milli: i64, cargo: CargoProfile) -> bool {
        quantity_milli > 0 && self.accepts_usage(cargo.usage(quantity_milli))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TradeShipment {
    pub id: u64,
    pub route_id: String,
    pub origin_market: String,
    pub destination_market: String,
    pub commodity_id: String,
    pub quantity_milli: i64,
    pub departure_tick: u64,
    pub eta_tick: u64,
    pub purchase_mcp: i64,
    pub freight_mcp: i64,
    pub expected_revenue_mcp: i64,
    pub expected_loss_mcp: i64,
    pub status: ShipmentStatus,
}

impl TradeShipment {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: u64,
        route_id: impl Into<String>,
        origin_market: impl Into<String>,
        destination_market: impl Into<String>,
        commodity_id: impl Into<String>,
        quantity_milli: i64,
        departure_tick: u64,
        eta_tick: u64,
        purchase_mcp: i64,
        freight_mcp: i64,
        expected_revenue_mcp: i64,
        expected_loss_mcp: i64,
    ) -> Self {
        assert!(quantity_milli > 0);
        assert!(eta_tick > departure_tick);
        assert!(purchase_mcp >= 0);
        assert!(freight_mcp >= 0);
        assert!(expected_revenue_mcp >= 0);
        assert!(expected_loss_mcp >= 0);
        Self {
            id,
            route_id: route_id.into(),
            origin_market: origin_market.into(),
            destination_market: destination_market.into(),
            commodity_id: commodity_id.into(),
            quantity_milli,
            departure_tick,
            eta_tick,
            purchase_mcp,
            freight_mcp,
            expected_revenue_mcp,
            expected_loss_mcp,
            status: ShipmentStatus::InTransit,
        }
    }

    pub fn progress_bps(&self, current_tick: u64) -> i64 {
        if current_tick <= self.departure_tick {
            return 0;
        }
        if current_tick >= self.eta_tick {
            return PROGRESS_BPS_MAX;
        }
        (((current_tick - self.departure_tick) as u128 * PROGRESS_BPS_MAX as u128)
            / (self.eta_tick - self.departure_tick) as u128) as i64
    }

    pub fn expected_profit_mcp(&self) -> i64 {
        self.expected_revenue_mcp - self.purchase_mcp - self.freight_mcp - self.expected_loss_mcp
    }

    pub(crate) fn append_stable_bytes(&self, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&self.id.to_le_bytes());
        append_string(bytes, &self.route_id);
        append_string(bytes, &self.origin_market);
        append_string(bytes, &self.destination_market);
        append_string(bytes, &self.commodity_id);
        bytes.extend_from_slice(&self.quantity_milli.to_le_bytes());
        bytes.extend_from_slice(&self.departure_tick.to_le_bytes());
        bytes.extend_from_slice(&self.eta_tick.to_le_bytes());
        bytes.extend_from_slice(&self.purchase_mcp.to_le_bytes());
        bytes.extend_from_slice(&self.freight_mcp.to_le_bytes());
        bytes.extend_from_slice(&self.expected_revenue_mcp.to_le_bytes());
        bytes.extend_from_slice(&self.expected_loss_mcp.to_le_bytes());
        bytes.push(match self.status {
            ShipmentStatus::Reserved => 0,
            ShipmentStatus::InTransit => 1,
            ShipmentStatus::Arrived => 2,
            ShipmentStatus::Lost => 3,
        });
    }
}

fn div_floor_i128(numerator: i128, denominator: i128) -> i64 {
    assert!(numerator >= 0 && denominator > 0);
    i64::try_from(numerator / denominator).expect("fixed-point logistics overflow")
}

fn div_round_i128(numerator: i128, denominator: i128) -> i64 {
    assert!(numerator >= 0 && denominator > 0);
    i64::try_from((numerator + denominator / 2) / denominator)
        .expect("fixed-point logistics overflow")
}

fn append_string(bytes: &mut Vec<u8>, value: &str) {
    let encoded = value.as_bytes();
    let len = u16::try_from(encoded.len()).expect("logistics string too long");
    bytes.extend_from_slice(&len.to_le_bytes());
    bytes.extend_from_slice(encoded);
}
