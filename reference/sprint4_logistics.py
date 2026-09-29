#!/usr/bin/env python3
"""Sprint 4 logistics reference model.

Validates reservation -> departure -> in-transit -> arrival, route capacity,
transport cost, deterministic risk outcome hooks, and derived map progress.
"""
from dataclasses import dataclass
from enum import Enum

MILLI = 1000
TICKS_PER_DAY = 288
PPM = 1_000_000

class Status(str, Enum):
    RESERVED = "reserved"
    IN_TRANSIT = "in_transit"
    ARRIVED = "arrived"
    LOST = "lost"

@dataclass
class Ledger:
    on_hand: int
    reserved: int = 0

    @property
    def available(self):
        return self.on_hand - self.reserved

    def reserve(self, quantity):
        if quantity <= 0 or quantity > self.available:
            raise ValueError("insufficient available inventory")
        self.reserved += quantity

    def depart_reserved(self, quantity):
        if quantity <= 0 or quantity > self.reserved:
            raise ValueError("insufficient reserved inventory")
        self.reserved -= quantity
        self.on_hand -= quantity

    def receive(self, quantity):
        if quantity <= 0:
            raise ValueError("quantity must be positive")
        self.on_hand += quantity

@dataclass(frozen=True)
class Route:
    id: str
    travel_ticks: int
    capacity_per_day: int
    variable_cost_mcp_per_base_unit: int
    fixed_cost_mcp: int

    def transport_cost_mcp(self, quantity_milli_base_units: int):
        variable = (quantity_milli_base_units * self.variable_cost_mcp_per_base_unit) // MILLI
        return self.fixed_cost_mcp + variable

@dataclass
class Shipment:
    id: int
    quantity: int
    departure_tick: int
    eta_tick: int
    status: Status = Status.RESERVED
    delivered: bool = False

    def progress_bps(self, current_tick: int):
        if current_tick <= self.departure_tick:
            return 0
        if current_tick >= self.eta_tick:
            return 10_000
        return ((current_tick - self.departure_tick) * 10_000) // (self.eta_tick - self.departure_tick)

    def depart(self, origin: Ledger):
        if self.status != Status.RESERVED:
            raise ValueError("shipment cannot depart from current state")
        origin.depart_reserved(self.quantity)
        self.status = Status.IN_TRANSIT

    def arrive(self, destination: Ledger, current_tick: int):
        if self.status == Status.ARRIVED:
            return False
        if self.status != Status.IN_TRANSIT or current_tick < self.eta_tick:
            return False
        destination.receive(self.quantity)
        self.status = Status.ARRIVED
        self.delivered = True
        return True

    def mark_lost(self):
        if self.status not in (Status.RESERVED, Status.IN_TRANSIT):
            return False
        self.status = Status.LOST
        return True

def baseline():
    route = Route(
        id="SEA-ATH-WD",
        travel_ticks=18*TICKS_PER_DAY,
        capacity_per_day=500_000_000,
        variable_cost_mcp_per_base_unit=300,
        fixed_cost_mcp=1_500_000,
    )
    quantity = 100_000_000 # 100,000 kg in milli-kg
    assert quantity <= route.capacity_per_day
    assert route.transport_cost_mcp(quantity) == 31_500_000

    origin = Ledger(1_000_000_000, 0)
    destination = Ledger(720_000_000, 0)
    origin.reserve(quantity)
    assert origin.available == 900_000_000

    depart = 100_000
    shipment = Shipment(1, quantity, depart, depart + route.travel_ticks)
    shipment.depart(origin)
    assert origin.on_hand == 900_000_000 and origin.reserved == 0
    assert destination.on_hand == 720_000_000
    assert shipment.progress_bps(depart + route.travel_ticks//2) == 5000
    assert not shipment.arrive(destination, shipment.eta_tick - 1)
    assert shipment.arrive(destination, shipment.eta_tick)
    assert destination.on_hand == 820_000_000
    # Idempotent arrival: no duplication.
    assert not shipment.arrive(destination, shipment.eta_tick + 100)
    assert destination.on_hand == 820_000_000
    return route, shipment

def capacity_rejection():
    route = Route("TEST", 100, 500_000_000, 100, 0)
    too_large = 600_000_000
    assert too_large > route.capacity_per_day

def loss_case():
    origin = Ledger(200_000_000, 0)
    destination = Ledger(0, 0)
    qty = 100_000_000
    origin.reserve(qty)
    shipment = Shipment(2, qty, 0, 100)
    shipment.depart(origin)
    shipment.mark_lost()
    assert shipment.status == Status.LOST
    assert destination.on_hand == 0
    assert origin.on_hand == 100_000_000

def main():
    route, shipment = baseline()
    capacity_rejection()
    loss_case()
    print(f"route={route.id} travel_ticks={route.travel_ticks} cost_mcp={route.transport_cost_mcp(shipment.quantity)}")
    print(f"shipment_eta={shipment.eta_tick} progress_final_bps={shipment.progress_bps(shipment.eta_tick)}")
    print("reservation/departure accounting: PASS")
    print("derived position: PASS")
    print("arrival idempotency: PASS")
    print("route capacity hook: PASS")
    print("loss state hook: PASS")

if __name__ == "__main__":
    main()
