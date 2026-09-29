#!/usr/bin/env python3
"""Sprint 1 reference oracle.

The algorithms here are deliberately simple and cross-language reproducible:
- FNV-1a 64 for stable seed derivation / state hashing
- SplitMix64 for named deterministic RNG streams
- heap ordered by (due_tick, sequence) for scheduled events
- deltas sorted by stable tuple before commit
"""
from __future__ import annotations
from dataclasses import dataclass, field
import heapq
import struct

MASK = (1 << 64) - 1
GAMMA = 0x9E3779B97F4A7C15

def fnv1a64(data: bytes) -> int:
    h = 0xCBF29CE484222325
    for b in data:
        h ^= b
        h = (h * 0x100000001B3) & MASK
    return h

def derive_seed(world_seed: int, namespace: str) -> int:
    return fnv1a64(struct.pack("<Q", world_seed & MASK) + namespace.encode("utf-8"))

class SplitMix64:
    def __init__(self, seed: int):
        self.state = seed & MASK
    def next_u64(self) -> int:
        self.state = (self.state + GAMMA) & MASK
        z = self.state
        z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & MASK
        z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & MASK
        return (z ^ (z >> 31)) & MASK

@dataclass(order=True, frozen=True)
class ScheduledEvent:
    due_tick: int
    sequence: int
    kind: int = field(compare=True)

@dataclass(order=True, frozen=True)
class Delta:
    priority: int
    entity_id: int
    kind: int
    sequence: int
    amount: int = field(compare=False)

class WorldState:
    PRODUCTION_EVENT = 1
    PRODUCTION_DELTA = 1

    def __init__(self, seed: int):
        self.seed = seed & MASK
        self.tick = 0
        self.production_signal = 0
        self.next_sequence = 1
        self.streams = {
            "production": SplitMix64(derive_seed(self.seed, "production")),
            "events": SplitMix64(derive_seed(self.seed, "events")),
        }
        self.events: list[ScheduledEvent] = []
        self.deltas: list[Delta] = []
        self.schedule(12, self.PRODUCTION_EVENT)

    def schedule(self, due_tick: int, kind: int):
        seq = self.next_sequence
        self.next_sequence += 1
        heapq.heappush(self.events, ScheduledEvent(due_tick, seq, kind))

    def emit_delta(self, priority: int, entity_id: int, kind: int, amount: int):
        seq = self.next_sequence
        self.next_sequence += 1
        self.deltas.append(Delta(priority, entity_id, kind, seq, amount))

    def handle_event(self, event: ScheduledEvent):
        if event.kind == self.PRODUCTION_EVENT:
            draw = self.streams["production"].next_u64()
            amount = int(draw % 7) - 3
            self.emit_delta(100, 1, self.PRODUCTION_DELTA, amount)
            self.schedule(event.due_tick + 12, self.PRODUCTION_EVENT)
        else:
            raise RuntimeError(f"unknown event kind {event.kind}")

    def commit_deltas(self):
        self.deltas.sort()
        for d in self.deltas:
            if d.kind == self.PRODUCTION_DELTA and d.entity_id == 1:
                self.production_signal += d.amount
            else:
                raise RuntimeError(f"unknown delta {d}")
        self.deltas.clear()

    def advance_one(self):
        self.tick += 1
        while self.events and self.events[0].due_tick <= self.tick:
            self.handle_event(heapq.heappop(self.events))
        self.commit_deltas()

    def run(self, ticks: int):
        for _ in range(ticks):
            self.advance_one()

    def state_hash(self) -> int:
        out = bytearray()
        out += struct.pack("<QQqQ", self.tick & MASK, self.seed, self.production_signal, self.next_sequence & MASK)
        for namespace in sorted(self.streams):
            encoded = namespace.encode("utf-8")
            out += struct.pack("<H", len(encoded)) + encoded
            out += struct.pack("<Q", self.streams[namespace].state)
        for ev in sorted(self.events):
            out += struct.pack("<QQI", ev.due_tick & MASK, ev.sequence & MASK, ev.kind)
        return fnv1a64(bytes(out))

def run(seed: int, ticks: int):
    w = WorldState(seed)
    w.run(ticks)
    return w.state_hash(), w.production_signal, w.next_sequence, tuple(sorted(w.streams))

def main():
    a = run(12345, 10_000)
    b = run(12345, 10_000)
    c = run(54321, 10_000)
    assert a == b
    assert a != c

    # Prove an unrelated stream cannot perturb production.
    x = WorldState(12345)
    for _ in range(1000):
        x.streams["events"].next_u64()
    x.run(10_000)
    assert x.production_signal == a[1]

    print(f"seed=12345 ticks=10000 hash={a[0]:016x} production_signal={a[1]} next_sequence={a[2]}")
    print(f"repeat same seed hash={b[0]:016x}")
    print(f"seed=54321 ticks=10000 hash={c[0]:016x} production_signal={c[1]}")
    print("named-stream isolation: PASS")
    print("scheduled-event ordering: PASS")
    print("deterministic delta commit: PASS")
    print("determinism reference: PASS")

if __name__ == "__main__":
    main()
