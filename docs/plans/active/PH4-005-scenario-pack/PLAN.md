# PH4-005 Plan

1. Add scenario-pack-v2 workspace crate.
2. Define serializable pack model and provenance statuses.
3. Add validation with deterministic error ordering.
4. Compile validated opening state into reducer commands.
5. Add a tiny generated Toril corridor fixture:
   - Waterdeep;
   - Neverwinter;
   - Luskan;
   - Grain;
   - Flour;
   - Preserved Fish;
   - Timber;
   - Iron.
6. Add tests for duplicates, dangling references, units, routes, reserved IDs, and deterministic initialization.
7. Add scenario-pack-validate CLI.
8. Add cargo xtask pack-validate command.
9. Run full repository CI.

Compatibility:
- sim-kernel-v2 remains independent of TorilGIS/source storage;
- v1 remains untouched.
