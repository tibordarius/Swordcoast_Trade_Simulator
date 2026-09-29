from __future__ import annotations

import hashlib
import json
import sqlite3
from dataclasses import dataclass, asdict

SCHEMA = r'''
PRAGMA foreign_keys = ON;
CREATE TABLE world (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  seed INTEGER NOT NULL,
  simulation_version TEXT NOT NULL,
  current_tick INTEGER NOT NULL
);
CREATE TABLE branch (
  id INTEGER PRIMARY KEY,
  world_id INTEGER NOT NULL REFERENCES world(id),
  name TEXT NOT NULL,
  canonical INTEGER NOT NULL DEFAULT 0,
  UNIQUE(world_id, name)
);
CREATE TABLE input_event_log (
  branch_id INTEGER NOT NULL REFERENCES branch(id),
  sequence INTEGER NOT NULL,
  tick_received INTEGER NOT NULL,
  actor TEXT NOT NULL,
  command_type TEXT NOT NULL,
  payload TEXT NOT NULL,
  schema_version INTEGER NOT NULL,
  event_hash TEXT NOT NULL,
  PRIMARY KEY(branch_id, sequence),
  UNIQUE(branch_id, event_hash)
);
CREATE TABLE snapshot (
  id INTEGER PRIMARY KEY,
  branch_id INTEGER NOT NULL REFERENCES branch(id),
  tick INTEGER NOT NULL,
  simulation_version TEXT NOT NULL,
  state_json TEXT NOT NULL,
  checksum TEXT NOT NULL,
  UNIQUE(branch_id, tick, checksum)
);
CREATE TRIGGER input_event_log_no_update
BEFORE UPDATE ON input_event_log
BEGIN
  SELECT RAISE(ABORT, 'input_event_log is append-only');
END;
CREATE TRIGGER input_event_log_no_delete
BEFORE DELETE ON input_event_log
BEGIN
  SELECT RAISE(ABORT, 'input_event_log is append-only');
END;
'''

@dataclass
class RefState:
    tick: int
    grain_milli: int
    cash_mcp: int
    route_risk_bps: int

    def canonical_json(self) -> str:
        return json.dumps(asdict(self), sort_keys=True, separators=(",", ":"))

    def checksum(self) -> str:
        return hashlib.sha256(self.canonical_json().encode()).hexdigest()


def event_hash(branch_id: int, sequence: int, tick: int, command: str, payload: dict) -> str:
    body = json.dumps({
        "branch_id": branch_id,
        "sequence": sequence,
        "tick": tick,
        "command": command,
        "payload": payload,
    }, sort_keys=True, separators=(",", ":"))
    return hashlib.sha256(body.encode()).hexdigest()


def apply_event(state: RefState, command: str, payload: dict) -> None:
    if command == "advance":
        state.tick += int(payload["ticks"])
    elif command == "grain_delta":
        state.grain_milli += int(payload["amount_milli"])
        assert state.grain_milli >= 0
    elif command == "cash_delta":
        state.cash_mcp += int(payload["amount_mcp"])
        assert state.cash_mcp >= 0
    elif command == "route_risk_set":
        state.route_risk_bps = int(payload["risk_bps"])
        assert 0 <= state.route_risk_bps <= 10_000
    else:
        raise AssertionError(command)


def append_event(db, branch_id, sequence, state, actor, command, payload):
    h = event_hash(branch_id, sequence, state.tick, command, payload)
    db.execute(
        "INSERT INTO input_event_log VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        (branch_id, sequence, state.tick, actor, command,
         json.dumps(payload, sort_keys=True), 1, h),
    )
    apply_event(state, command, payload)


def replay(db, branch_id, snapshot_state, after_sequence=0):
    state = RefState(**asdict(snapshot_state))
    rows = db.execute(
        "SELECT sequence, command_type, payload FROM input_event_log "
        "WHERE branch_id=? AND sequence>? ORDER BY sequence",
        (branch_id, after_sequence),
    ).fetchall()
    for _, command, payload in rows:
        apply_event(state, command, json.loads(payload))
    return state


def main():
    db = sqlite3.connect(":memory:")
    db.executescript(SCHEMA)
    db.execute("INSERT INTO world VALUES (1, 'Sword Coast', 12345, '0.1.0', 0)")
    db.execute("INSERT INTO branch VALUES (1, 1, 'MAIN', 1)")

    state = RefState(tick=0, grain_milli=2_000_000_000, cash_mcp=900_000_000, route_risk_bps=250)
    append_event(db, 1, 1, state, "dm", "advance", {"ticks": 288})
    append_event(db, 1, 2, state, "system", "grain_delta", {"amount_milli": -125_000_000})

    snap = RefState(**asdict(state))
    db.execute(
        "INSERT INTO snapshot(branch_id,tick,simulation_version,state_json,checksum) VALUES (?,?,?,?,?)",
        (1, snap.tick, '0.1.0', snap.canonical_json(), snap.checksum()),
    )

    append_event(db, 1, 3, state, "dm", "route_risk_set", {"risk_bps": 900})
    append_event(db, 1, 4, state, "system", "cash_delta", {"amount_mcp": 55_000_000})
    append_event(db, 1, 5, state, "system", "grain_delta", {"amount_milli": 300_000_000})

    restored = replay(db, 1, snap, after_sequence=2)
    assert restored == state
    assert restored.checksum() == state.checksum()

    # Event log must reject mutation.
    try:
        db.execute("UPDATE input_event_log SET actor='tampered' WHERE branch_id=1 AND sequence=1")
        raise AssertionError("append-only event log accepted UPDATE")
    except sqlite3.DatabaseError as exc:
        assert "append-only" in str(exc)

    # A second reconstruction from the stored snapshot text must also match.
    stored_json, stored_checksum = db.execute(
        "SELECT state_json, checksum FROM snapshot WHERE branch_id=1 ORDER BY tick DESC LIMIT 1"
    ).fetchone()
    assert hashlib.sha256(stored_json.encode()).hexdigest() == stored_checksum
    stored_state = RefState(**json.loads(stored_json))
    restored2 = replay(db, 1, stored_state, after_sequence=2)
    assert restored2 == state

    print("SPRINT7_PERSISTENCE: PASS")
    print("final_tick", state.tick)
    print("final_grain_milli", state.grain_milli)
    print("final_cash_mcp", state.cash_mcp)
    print("route_risk_bps", state.route_risk_bps)
    print("state_sha256", state.checksum())

if __name__ == "__main__":
    main()
