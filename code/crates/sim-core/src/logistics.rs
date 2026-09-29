use crate::InventoryLedger;

pub const MILLI_BASE_UNIT: i64 = 1_000;
pub const PROGRESS_BPS_MAX: i64 = 10_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShipmentStatus {
    Reserved,
    InTransit,
    Arrived,
    Lost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
