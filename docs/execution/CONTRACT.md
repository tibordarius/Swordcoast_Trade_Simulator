# Reconciled Toril execution contract 0.1

Status: independently reviewed design proposal, 3 October 2026. This specifies decisions for subsequent implementation; it does not change accepted repository ADRs or claim these interfaces exist. Repository source and tests remain executable truth: see ../../ARCHITECTURE.md, ../decisions/, ../../code/crates/sim-kernel-v2/src/ and ../../code/crates/sim-kernel-v2/tests/. This document reconciles the reviewed behavior and persistence proposals and their six review decisions; those research drafts are not normative repository interfaces. Where the proposals differ, this document governs the proposed 0.1 design. Format-6 behavior remains the compatibility reference until a versioned implementation is accepted.

## 1. Authority and reproducibility envelope

Only WorldReducer commits causal state. Storage stages and publishes committed results; it does not mutate balances independently. GIS/lore authoring compiles immutable ScenarioPack inputs. UI reads projections and submits commands. Agent-generated drafts are reviewed data inputs, never live runtime economic decisions.

An ExecutionManifest retains exact pack bytes and raw-byte SHA256, pack schema/revision, source provenance/continuity profile, engine commit and recoverable executable, dependency lock, compiler/toolchain, command/event/state codec, arithmetic/rounding, RNG and projection versions. Whitespace changes make a different raw pack artifact; semantic identity may be recorded separately but cannot substitute for retained bytes. The compiled registry gets a canonical ordered-record digest under a named codec. It is outside current WorldState, so a checkpoint must authenticate it explicitly.

The ordinary branch keeps its registry/manifest immutable. A changed registry or incompatible rule/engine migration creates a reviewed new lineage. Inspecting recorded history may work without an old executable; recomputation must fail visibly if the retained executable/context cannot be recovered. FNV format-6 state_hash remains a regression fingerprint, not archival integrity proof.

## 2. Admission identities, retries and cursors

Separate these identities: lineage, branch, stable request, successful kernel sequence, durable admission sequence, world revision, scheduler insertion sequence and event generation. Never infer one from another.

The durable request key is (branch_id, request_id). Transaction, trade and observation identities are unique over a branch's visible history, including its immutable inherited prefix. Descendants cannot reuse an inherited ID. Independent sibling suffixes may use the same textual domain ID under different branch contexts; this is not a lineage-global lock. Shared-prefix outcomes retain their original branch identity and are referenced rather than republished. A canonical business digest includes principal/authorization context, operation, arguments and requested effective tick. Retry transport metadata and expected_head are excluded. First look up a known request. Identical business digest returns its recorded receipt even if its original head is now stale. Different digest returns IdempotencyConflict before checking head. A new request with a reused TransactionId remains a domain duplicate error.

For an unknown request, compare expected_head with the last successful kernel head under a serialized writer/CAS. HeadConflict is a non-admitted precondition failure: no state effects and no durable request reservation. After a matching head, deterministic domain rejection receives a durable immutable receipt and consumes an admission sequence; it does not advance successful kernel sequence, world revision or scheduler. Infrastructure failure is not a durable domain rejection. Retrying a recorded rejection returns that rejection; intentional later attempts use a new request ID.

Durable admission sequences are contiguous and cover successful and domain-rejected receipts. Successful kernel sequences are separately contiguous; only committed successes enter the kernel replay suffix. A candidate sequence used during a rejected staged call does not become its applied cursor. Initialization establishes an explicit kernel cursor from the compiled command list.

A checkpoint records last admission cursor, successful kernel cursor, outcome-journal position/commitment and expected branch head. Restore verifies continuity after each cursor, rejects duplicates/missing committed records and leaves an empty suffix unchanged. Current replay's increasing-within-slice check is insufficient. Domain identity receipts remain durable beyond hot-cache compaction.

## 3. Command and publication atomicity

One AdvanceTo remains one atomic external command, even when it fires many events. Stage state and outcomes; publish request receipt, successful head and all causal outcomes together. A failed due event rolls back the whole advance: balances, tick/revision, queue/generations, knowledge and staged outcomes all remain unchanged. Never append to the durable journal during speculative reduction.

Crash before publication produces no committed success. Crash after commit but before acknowledgement yields the old receipt on retry. Each output is identified by successful kernel sequence plus stable outcome ordinal, with event ID/generation/causal parent where applicable. Ledger positions alone are not universal outcome identities. Read projections are rebuildable and cannot be the only retained causal evidence.

## 4. Time, ordering and intervention timing

Keep current event order (tick, domain priority, insertion sequence), including System0, Production10, Population20, Logistics30, Information40 and Shock50. Keep the current dynamic drain: newly inserted due work is eligible for the next minimum-queue pop, even if its domain priority is earlier than an event already processed. This is deterministic queue ordering, not a strict global phase barrier. New payloads need termination/positive-delay rules and adversarial tests.

An immediate route closure takes effect at the current tick when its ordered command is applied, before the next advance. It denies subsequent departures, but does not retroactively cancel a leg already moving. A scheduled Shock50 closure at T follows same-tick logistics. Do not label it closure-before-logistics. A future closure that must precede all logistics at T needs an explicitly versioned control barrier/ordering change, outside 0.1 compatibility. Requested effective dates cannot silently imply such a barrier.

Interventions have stable IDs and separate base access from active closure overlays. Access is base-open AND no active closure. Expiry/reopening removes the named intervention; it cannot remove another active closure. SetRouteOpen in the incomplete logistics branch can express base access but must not become a bypass of later overlays. Same-tick intervention ties use recorded insertion order, never wall time.

Calendar/tick configuration is explicit and retained: calendar rules, epoch, continuity profile and ticks per period. Host timezone and the generated 1492 DR fixture do not choose the campaign date.

## 5. Ownership, reservations and cancellation

Goods have one commodity/quantity, legal owner, custody/location and allocation state. Ownership transfer and physical movement are distinct. Stock in a reserved lot, WIP or transit is counted once and cannot be sold/consumed/reserved again. Account totals reconcile to physical holdings and explicit source/sink postings.

Protect reserved/WIP debits through reducer-validated operation identity and allocation metadata. A dedicated Holding account alone is insufficient because current ApplyTransaction can debit it. Start future multi-input production only after all inputs, labour and capacity can be reserved atomically. Capacity keys specify whether they represent a route constraint, carrier or terminal; they are not interchangeable.

For initial protected semantics, generic CancelEvent is limited to events with no economic unwind, such as Noop or information delivery. Cancelling information consumes its observation identity and does not reconstruct a later quote. Reject generic cancellation of production completion, consumption recurrence or arrival. Generic ScheduleEvent also cannot allocate a reserved economic event identity or replace an economic event with Noop or another payload. A reducer-owned event identity/domain-operation registry validates allocation and replacement; a caller-supplied domain label or prefix is not authority. Economic replacements require the validated owning domain operation and atomic commitment updates. This closes the generation-replacement bypass in the current generic scheduler. Production cancellation after start is rejected until a dedicated tested refund/stranded-WIP operation exists. Route interruption/reroute is a dedicated economic command with explicit custody, reservation, obligation and replacement-event effects.

Already-moving cargo completes its current leg under the default closure policy. A closed next leg strands it at a transfer location. Destination terminal closure separately blocks handling; it does not erase or teleport goods. Arrival requires the expected shipment, leg/status and movement generation. Reroute atomically invalidates the obsolete event and creates a complete replacement/stranded result. One arrival or cancellation never settles cargo/payment twice.

## 6. Complete checkpoint and bounded active state

Checkpoint every live commitment: accounts/balances, allocation and WIP, cargo/custody/legs, carrier and route capacity, terminal queues, orders/pipeline obligations, unpaid settlements, production/population state, actor knowledge and immutable pending observations, scheduler ordering/generations, active interventions/conflicts, RNG cursors where used, and durable sequence/index watermarks. Add a feature's fields before accepting that feature.

Current format 6 includes growing ledger/trade/consumption/observation/fired-event histories and ID sets; it is not bounded active state. A later version separates canonical active-state commitment from ordered durable-journal commitment. No silent pruning under the old hash/codec contract.

Do not discard causal history merely because a record is old: scarcity currently reads recent unmet-consumption windows. Retain bounded rolling data sufficient for every active configured window, with identical inclusive-boundary arithmetic. Completed production and cancelled scheduler tombstones need explicit safe compaction rules. Keep identity decisions durably and use bounded caches. If uniqueness checks move outside the pure kernel, retain their deterministic admission evidence; do not introduce timing-dependent database reads inside reducer execution. This interface change requires its own accepted design and migration before compaction ships.

## 7. Branches, date navigation and compatibility

Fork at an exact successful-command boundary with an authenticated immutable prefix. Parent commands/outcomes never change. A correction inside a long advance forks before that command, regenerates a child advance to the chosen tick, applies the intervention and recomputes its suffix. Do not reuse the parent's future outcomes as recomputed results or fork halfway through its atomic commit.

Recorded historical inspection and engine recomputation are different operations. Advancing, seeking a recorded date, previewing a child and promoting campaign authority have distinct commands/queries. Preview does not mutate the active campaign. Splitting advances can alter sequence/revision trace; only identical command streams promise identical complete state hashes. Branch comparison aligns economic dates and shared exogenous conditions rather than assuming identical administrative revisions.

Authored events with incompatible prerequisites emit a durable Suspended/conflict outcome instead of silently retargeting or failing the entire advance. Resolution is an explicit skip/retarget/replace command with author/reason. Active conflicts are checkpointed. No automatic economic rescue is implied.

## 8. RNG and arithmetic

Use a versioned, length-framed structured key: process kind, stable entity/region, occurrence/time bucket and draw ordinal. Share world seed and region/time keys for exogenous weather across sibling branches, excluding branch ID and scheduler order. Retain shared-prefix entity identities for endogenous shipment/leg comparisons; newly created branch-specific entities may differ. Mutable stream cursors must be checkpointed if used. Current arbitrary string seeding primitive is not this catalog.

Preserve integral MoneyCp/Quantity and explicitly recorded UnitPrice scales. Current signed division truncates toward zero; positive trade values floor fractional copper. Arithmetic changes, rounding residue accounting, conversion scales or integer-width changes require a versioned contract and overflow/boundary tests. Production transforms commodities through explicit source/sinks; do not assert conservation of unlike units as simple quantity equality.

## Acceptance and readiness

ACCEPTANCE.csv turns these decisions into observable scenarios. Existing baseline tests cover implemented foundations; future cases remain NOT IMPLEMENTED. The independent review's six ambiguities are resolved here, but runtime behavior is not altered. Acceptance requires direct-versus-restored complete active state plus ordered outcomes, not balances alone, and final combined-head checks.

NEXT.csv assigns the bounded follow-on jobs and retains their R-series dependency gates; ROADMAP.csv retains the complete proposed R-series roadmap and dependency DAG. Normalize formatting separately, retain a reproducible build manifest, and reconcile the partial PH4-009 branch before implementation delegation. Full household economics, owned carriers/terminals, durable branches and century validation remain later acceptance gates.

## Repository adoption and parallel-workflow status

This file is Proposed Contracts 0.1, independently reviewed as design only. Adoption into the repository makes it reviewable; it does not accept a new ADR, implement the proposed interfaces, or supersede accepted ADR-0001 through ADR-0003. The scoped adoption task is ../plans/active/BASE-001-reproducible-contracts/.

The coordinator retains contract reconciliation, shared interfaces, final integration and publication. Initially use at most three independently reviewable workers after prerequisites are available, with exclusive write paths and explicit semantic ownership. Workers do not recursively delegate. Repository instructions and the five templates under ../templates/ remain authoritative. Separate branches/worktrees are preferred; shared worktrees require exclusive path ownership and isolated mutable test artifacts. Reducer/state/command/IDs/registry/scheduler/snapshots, monetary primitives, pack schema, fixtures, migrations, Cargo/lockfile, CI and root instructions remain integrator-owned unless explicitly leased.

Every dispatch records task/attempt ID, pinned input commit, contract status/version, fixture identity, dependency status, allowed paths, acceptance cases and stop conditions. Use READY, IN_PROGRESS, VERIFY, REVIEW, BLOCKED and DONE; attempt revocation is separate. Inspect stale or partial branches before reuse, preserve diagnostics, and reject results from revoked attempts. An independent reviewer reports concrete triggers and missing evidence without editing the reviewed patch. Integrate serially, rerun affected checks and final combined-head gates, and record exact commands, working directories, environment, result and evidence location. No serial-versus-parallel performance claim is made.

ACCEPTANCE.csv's implemented rows describe existing baseline foundations from the reviewed research, not new execution evidence for this task. Proposed rows remain NOT IMPLEMENTED until direct evidence establishes them. NEXT.csv and ROADMAP.csv statuses are planning statuses from that review, not proof of satisfied dependencies. Audit current code and checks before marking a follow-on job ready or done. Campaign authority, household economics, owned carriers/terminals, durable history and century validation remain later gates.
