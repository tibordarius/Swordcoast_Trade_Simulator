from pathlib import Path
import re

root = Path(__file__).resolve().parents[1] / "code" / "migrations"
schema_files = sorted(root.glob("0*.sql"))
assert schema_files, "no schema migrations found"
text = "\n".join(p.read_text() for p in schema_files)
required = [
    "world", "branch", "settlement", "commodity", "market", "market_inventory",
    "market_state", "route", "shipment", "input_event_log", "snapshot"
]
for name in required:
    assert re.search(rf"CREATE TABLE\s+{re.escape(name)}\b", text, re.I), f"missing table {name}"
for forbidden in [r"\bREAL\b", r"\bDOUBLE\s+PRECISION\b"]:
    assert not re.search(forbidden, text, re.I), f"forbidden floating economic type matched: {forbidden}"
assert "CREATE EXTENSION IF NOT EXISTS postgis" in text
assert "input_event_log_no_update" in text and "input_event_log_no_delete" in text
assert "price_mcp" in text and "quantity_milli" in text
print(f"MIGRATION_LINT: PASS ({len(schema_files)} schema migrations)")
