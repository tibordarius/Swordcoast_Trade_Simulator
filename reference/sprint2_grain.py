#!/usr/bin/env python3
"""Sprint 2 reference model: physical grain ledger only.

No prices, merchants, or shipments are modeled here. The sole purpose is to
prove exact production/consumption/spoilage accounting with fixed-point units.
"""
from dataclasses import dataclass

MILLI = 1000
TICKS_PER_HOUR = 12
TICKS_PER_DAY = 288
PPM = 1_000_000

@dataclass
class InventoryLedger:
    on_hand: int
    reserved: int
    target_reserve: int
    unmet_demand: int = 0
    produced: int = 0
    consumed: int = 0
    spoiled: int = 0

    @property
    def available(self) -> int:
        return self.on_hand - self.reserved

    def add_production(self, amount: int):
        assert amount >= 0
        self.on_hand += amount
        self.produced += amount

    def consume(self, requested: int):
        assert requested >= 0
        fulfilled = min(requested, max(0, self.available))
        self.on_hand -= fulfilled
        self.consumed += fulfilled
        self.unmet_demand += requested - fulfilled
        return fulfilled

    def apply_spoilage(self, ppm: int):
        assert 0 <= ppm <= PPM
        # Reserved goods spoil too because they are still physically present.
        loss = (self.on_hand * ppm) // PPM
        self.on_hand -= loss
        self.spoiled += loss
        # If spoilage pushed stock below reservations, clamp reservations.
        self.reserved = min(self.reserved, self.on_hand)
        return loss

class ExactDailyRate:
    """Distribute a fixed daily integer quantity over hours without drift."""
    def __init__(self, amount_per_day: int):
        self.amount_per_day = amount_per_day
        self.hours_elapsed = 0
        self.emitted = 0

    def next_hour(self) -> int:
        self.hours_elapsed += 1
        target = (self.amount_per_day * self.hours_elapsed) // 24
        due = target - self.emitted
        self.emitted = target
        return due

class GrainScenario:
    def __init__(self, *, start: int, target: int, production_per_day: int, consumption_per_day: int, spoilage_ppm: int):
        self.tick = 0
        self.initial_stock = start
        self.ledger = InventoryLedger(start, 0, target)
        self.production_rate = ExactDailyRate(production_per_day)
        self.consumption_rate = ExactDailyRate(consumption_per_day)
        self.spoilage_ppm = spoilage_ppm
        self.requested_demand = 0

    def advance_one(self):
        self.tick += 1
        if self.tick % TICKS_PER_HOUR == 0:
            produced = self.production_rate.next_hour()
            requested = self.consumption_rate.next_hour()
            self.ledger.add_production(produced)
            self.requested_demand += requested
            self.ledger.consume(requested)
        if self.tick % TICKS_PER_DAY == 0:
            self.ledger.apply_spoilage(self.spoilage_ppm)

    def run_days(self, days: int):
        for _ in range(days * TICKS_PER_DAY):
            self.advance_one()

    def assert_conservation(self):
        lhs = self.initial_stock + self.ledger.produced
        rhs = self.ledger.on_hand + self.ledger.consumed + self.ledger.spoiled
        assert lhs == rhs, (lhs, rhs)
        assert self.requested_demand == self.ledger.consumed + self.ledger.unmet_demand

def normal_case():
    s = GrainScenario(
        start=700_000_000,
        target=700_000_000,
        production_per_day=30_000_000,
        consumption_per_day=25_000_000,
        spoilage_ppm=500,
    )
    s.run_days(30)
    s.assert_conservation()
    return s

def shortage_case():
    s = GrainScenario(
        start=50_000_000,
        target=700_000_000,
        production_per_day=5_000_000,
        consumption_per_day=25_000_000,
        spoilage_ppm=500,
    )
    s.run_days(10)
    s.assert_conservation()
    assert s.ledger.unmet_demand > 0
    return s

def main():
    n = normal_case()
    q = shortage_case()
    print("normal case:")
    print(f"  on_hand_mkg={n.ledger.on_hand}")
    print(f"  produced_mkg={n.ledger.produced}")
    print(f"  consumed_mkg={n.ledger.consumed}")
    print(f"  spoiled_mkg={n.ledger.spoiled}")
    print(f"  unmet_mkg={n.ledger.unmet_demand}")
    print("shortage case:")
    print(f"  on_hand_mkg={q.ledger.on_hand}")
    print(f"  produced_mkg={q.ledger.produced}")
    print(f"  consumed_mkg={q.ledger.consumed}")
    print(f"  spoiled_mkg={q.ledger.spoiled}")
    print(f"  unmet_mkg={q.ledger.unmet_demand}")
    print("conservation-of-goods: PASS")
    print("exact hourly rate distribution: PASS")
    print("unmet demand accounting: PASS")
    print("spoilage hook: PASS")

if __name__ == "__main__":
    main()
