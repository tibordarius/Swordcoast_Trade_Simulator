# Parallel work on Toril

Use one coordinator/integrator and initially at most three concurrent workers for independent bounded jobs. Requirements, task dependencies, shared contracts, integration and publication belong to the coordinator. A larger team needs measured benefit in accepted throughput or turnaround within the cost budget.

Read root AGENTS.md, architecture, accepted ADRs and the active task before dispatch. Preserve SPEC, PLAN, ACCEPTANCE, HANDOFF and CHECKPOINT. The reviewed execution proposal is in ../execution/CONTRACT.md; it is not an implemented runtime contract or an accepted replacement for existing ADRs.

## Dispatch and ownership

Each task receives a task/attempt ID, pinned base, contract/input versions, dependency evidence, concrete exclusive write paths, semantic interface ownership, acceptance cases, checks, checkpoint and stop condition. File separation alone does not separate a shared invariant. Missing predecessors or incompatible contracts make the task BLOCKED.

Implementation workers use separate branches/worktrees where supported. Hosted workers may share a filesystem; the coordinator must assign exclusive paths. Do not share mutable test databases or build output directories across workers without explicit coordination.

Reserve reducer/state/command/ID/registry/scheduler/snapshot, ledger/value primitives, ScenarioPack schema/version, shared fixtures, Cargo metadata, migrations, CI and root instructions to the integration owner. Workers propose shared edits; the owner applies them serially. A temporary exclusive lease may be assigned explicitly.

Workers cannot recursively delegate, merge or publish under this starting policy. Reviewers inspect the concrete result read-only and return findings; they do not fix what they review. Existing permissions and authorization rules still apply.

## Review and integration

1. Collect the concrete patch, owned-path list, base/head and acceptance evidence.
2. Have an independent reviewer challenge applicable causal cases and invariants.
3. Integrate one patch at a time, rebase dependent work and rerun affected checks.
4. Run the prescribed combined-head gate before merging. Missing database/service checks are NOT RUN, even when a test returns early and reports success.
5. Record exact command, cwd, commit, environment and result in the task handoff.

READY precedes dispatch, then IN_PROGRESS, VERIFY, REVIEW and DONE under the existing harness. Integration-ready evidence does not redefine READY. DONE requires acceptance and the integrated result. A task can finish audit/check classification without establishing a fully green implementation baseline, but must state that distinction.

At checkpoints return evidence, changes, checks, blockers and next action. Revoke failed attempts explicitly and quarantine partial output. Late output from a revoked attempt cannot integrate. An old-base pass certifies only that old base. If a contract changes, pause affected descendants and reconcile them.

## Model and role configuration

The optional local .codex files define toril_researcher, toril_implementer and toril_reviewer using gpt-6.1-sol. Medium is the starting effort for bounded work; high is the starting review effort. These are project defaults to evaluate, not a documented optimum. The project config caps spawned threads at three, excluding the primary.

Local Codex reads project configuration only when the project is trusted. Hosted ChatGPT Work does not read local Codex TOML; explicitly request the model, bounded parallel jobs and consolidated review in that interface. Do not claim these files configure a hosted Work chat. Current account/client availability governs exposed models and effort controls.

For research, distinguish dated evidence, inference and campaign overrides. For implementation, return patches and exact checks. For review, give concrete triggers and evidence, distinguish confirmed defects from hypotheses and state unverified scope. Same-model review can share blind spots; executable adversarial checks remain necessary.

## Current work split

../execution/ROADMAP.csv is the full R-series research dependency snapshot. ../execution/NEXT.csv is the bounded follow-on list. Contract/data/household/production drafting can run independently after the baseline/adoption gate. Logistics or persistence prototypes before their original implementation predecessors are labelled spikes and do not complete gated R3/R4 tasks.

Compare serial and parallel matched task slices using the same model/input/rubric. Include coordination, review, retries and integration in elapsed time and token/cost totals. Track accepted output, defects, rework, conflicts and missing evidence. Worker count and code volume are not success measures.

Official configuration guidance checked 2026-10-03:

- https://learn.chatgpt.com/docs/agent-configuration/subagents
- https://learn.chatgpt.com/docs/config-file/config-reference
- https://learn.chatgpt.com/docs/developer-settings
- https://developers.openai.com/api/docs/models/gpt-6.1-sol
