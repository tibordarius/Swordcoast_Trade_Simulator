use sim_kernel_v2::{
    decode_snapshot, encode_snapshot, replay, state_hash, ApplyError, Command, CommandEnvelope,
    CommodityId, EconomicTransaction, InventoryAccountId, InventoryAccountKind, InventoryPosting,
    PopulationCohort, PopulationCohortId, ProductionBatchId, ProductionBatchStatus,
    ProductionRecipe, ProductionSite, ProductionSiteId, Quantity, RecipeId, SimTick, TransactionId,
    WorldReducer, WorldState,
};

struct Harness {
    state: WorldState,
    sequence: u64,
}

impl Harness {
    fn new(raw_grain: i64, food_grain: i64) -> Self {
        let mut harness = Self {
            state: WorldState::new(42),
            sequence: 0,
        };

        for (id, kind) in [
            ("inventory.raw", InventoryAccountKind::Holding),
            ("inventory.wip", InventoryAccountKind::Holding),
            ("inventory.output", InventoryAccountKind::Holding),
            ("inventory.food", InventoryAccountKind::Holding),
            ("inventory.drain", InventoryAccountKind::Holding),
            ("system.production", InventoryAccountKind::SourceOrSink),
            ("system.consumption", InventoryAccountKind::SourceOrSink),
            ("system.seed", InventoryAccountKind::SourceOrSink),
        ] {
            harness.apply(Command::OpenInventoryAccount {
                account_id: InventoryAccountId::new(id),
                kind,
            })
            .unwrap();
        }

        if raw_grain != 0 {
            harness
                .apply(Command::ApplyTransaction {
                    transaction: inventory_transfer(
                        "seed.raw",
                        "system.seed",
                        "inventory.raw",
                        "commodity.grain",
                        raw_grain,
                    ),
                })
                .unwrap();
        }

        if food_grain != 0 {
            harness
                .apply(Command::ApplyTransaction {
                    transaction: inventory_transfer(
                        "seed.food",
                        "system.seed",
                        "inventory.food",
                        "commodity.grain",
                        food_grain,
                    ),
                })
                .unwrap();
        }

        harness
    }

    fn apply(&mut self, command: Command) -> Result<(), ApplyError> {
        self.sequence += 1;
        WorldReducer::apply(
            &mut self.state,
            &CommandEnvelope::new(self.sequence, command),
        )
    }

    fn register_flour_production(&mut self) {
        self.apply(Command::RegisterProductionRecipe {
            recipe: flour_recipe(),
        })
        .unwrap();

        self.apply(Command::RegisterProductionSite {
            site: flour_site(),
        })
        .unwrap();
    }
}

fn inventory_transfer(
    transaction_id: &str,
    from: &str,
    to: &str,
    commodity: &str,
    quantity: i64,
) -> EconomicTransaction {
    EconomicTransaction::new(
        TransactionId::new(transaction_id),
        vec![
            InventoryPosting::new(
                InventoryAccountId::new(from),
                CommodityId::new(commodity),
                Quantity::new(-quantity),
            ),
            InventoryPosting::new(
                InventoryAccountId::new(to),
                CommodityId::new(commodity),
                Quantity::new(quantity),
            ),
        ],
        vec![],
    )
}

fn flour_recipe() -> ProductionRecipe {
    ProductionRecipe::new(
        RecipeId::new("recipe.flour"),
        CommodityId::new("commodity.grain"),
        Quantity::new(10),
        CommodityId::new("commodity.flour"),
        Quantity::new(8),
    )
}

fn flour_site() -> ProductionSite {
    ProductionSite::new(
        ProductionSiteId::new("site.waterdeep.mill"),
        RecipeId::new("recipe.flour"),
        InventoryAccountId::new("inventory.raw"),
        InventoryAccountId::new("inventory.wip"),
        InventoryAccountId::new("inventory.output"),
        InventoryAccountId::new("system.production"),
        10,
    )
}

fn food_cohort() -> PopulationCohort {
    PopulationCohort::new(
        PopulationCohortId::new("cohort.waterdeep.common"),
        1_000,
        InventoryAccountId::new("inventory.food"),
        InventoryAccountId::new("system.consumption"),
        CommodityId::new("commodity.grain"),
        Quantity::new(60),
        10,
    )
}

fn grain() -> CommodityId {
    CommodityId::new("commodity.grain")
}

fn flour() -> CommodityId {
    CommodityId::new("commodity.flour")
}

fn balance(state: &WorldState, account: &str, commodity: &CommodityId) -> Quantity {
    state
        .inventory_balance(&InventoryAccountId::new(account), commodity)
        .expect("test account must exist")
}

#[test]
fn production_reserves_input_then_completes_only_at_scheduled_tick() {
    let mut harness = Harness::new(100, 0);
    harness.register_flour_production();

    harness
        .apply(Command::StartProductionBatch {
            batch_id: ProductionBatchId::new("batch.001"),
            site_id: ProductionSiteId::new("site.waterdeep.mill"),
        })
        .unwrap();

    assert_eq!(balance(&harness.state, "inventory.raw", &grain()), Quantity::new(90));
    assert_eq!(balance(&harness.state, "inventory.wip", &grain()), Quantity::new(10));
    assert_eq!(
        balance(&harness.state, "inventory.output", &flour()),
        Quantity::ZERO
    );
    assert_eq!(
        harness
            .state
            .production_batch(&ProductionBatchId::new("batch.001"))
            .unwrap()
            .status(),
        ProductionBatchStatus::InProgress
    );
    assert_eq!(harness.state.scheduler().active_event_count(), 1);

    harness
        .apply(Command::AdvanceTo {
            tick: SimTick::new(9),
        })
        .unwrap();

    assert_eq!(balance(&harness.state, "inventory.wip", &grain()), Quantity::new(10));
    assert_eq!(
        balance(&harness.state, "inventory.output", &flour()),
        Quantity::ZERO
    );

    harness
        .apply(Command::AdvanceTo {
            tick: SimTick::new(10),
        })
        .unwrap();

    assert_eq!(balance(&harness.state, "inventory.wip", &grain()), Quantity::ZERO);
    assert_eq!(
        balance(&harness.state, "inventory.output", &flour()),
        Quantity::new(8)
    );
    assert_eq!(
        balance(&harness.state, "system.production", &grain()),
        Quantity::new(10)
    );
    assert_eq!(
        balance(&harness.state, "system.production", &flour()),
        Quantity::new(-8)
    );
    assert_eq!(
        harness
            .state
            .production_batch(&ProductionBatchId::new("batch.001"))
            .unwrap()
            .status(),
        ProductionBatchStatus::Completed
    );
    assert_eq!(harness.state.scheduler().active_event_count(), 0);
}

#[test]
fn insufficient_input_rejects_batch_without_partial_state() {
    let mut harness = Harness::new(5, 0);
    harness.register_flour_production();

    let before = state_hash(&harness.state).unwrap();

    let error = harness
        .apply(Command::StartProductionBatch {
            batch_id: ProductionBatchId::new("batch.insufficient"),
            site_id: ProductionSiteId::new("site.waterdeep.mill"),
        })
        .unwrap_err();

    assert!(matches!(
        error,
        ApplyError::NegativeInventoryBalance { .. }
    ));
    assert_eq!(before, state_hash(&harness.state).unwrap());
    assert!(
        harness
            .state
            .production_batch(&ProductionBatchId::new("batch.insufficient"))
            .is_none()
    );
    assert_eq!(harness.state.scheduler().active_event_count(), 0);
}

#[test]
fn recurring_consumption_records_served_and_unmet_per_cycle() {
    let mut harness = Harness::new(0, 100);

    harness
        .apply(Command::RegisterPopulationCohort {
            cohort: food_cohort(),
        })
        .unwrap();

    harness
        .apply(Command::AdvanceTo {
            tick: SimTick::new(10),
        })
        .unwrap();
    harness
        .apply(Command::AdvanceTo {
            tick: SimTick::new(20),
        })
        .unwrap();
    harness
        .apply(Command::AdvanceTo {
            tick: SimTick::new(30),
        })
        .unwrap();

    assert_eq!(balance(&harness.state, "inventory.food", &grain()), Quantity::ZERO);
    assert_eq!(
        balance(&harness.state, "system.consumption", &grain()),
        Quantity::new(100)
    );

    let records = harness.state.consumption_records();
    assert_eq!(records.len(), 3);

    assert_eq!(records[0].cycle(), 1);
    assert_eq!(records[0].requested(), Quantity::new(60));
    assert_eq!(records[0].served(), Quantity::new(60));
    assert_eq!(records[0].unmet(), Quantity::ZERO);

    assert_eq!(records[1].cycle(), 2);
    assert_eq!(records[1].served(), Quantity::new(40));
    assert_eq!(records[1].unmet(), Quantity::new(20));

    assert_eq!(records[2].cycle(), 3);
    assert_eq!(records[2].served(), Quantity::ZERO);
    assert_eq!(records[2].unmet(), Quantity::new(60));

    assert_eq!(harness.state.scheduler().active_event_count(), 1);
}

#[test]
fn advance_is_atomic_when_due_event_fails() {
    let mut harness = Harness::new(100, 0);
    harness.register_flour_production();

    harness
        .apply(Command::StartProductionBatch {
            batch_id: ProductionBatchId::new("batch.atomic"),
            site_id: ProductionSiteId::new("site.waterdeep.mill"),
        })
        .unwrap();

    harness
        .apply(Command::ApplyTransaction {
            transaction: inventory_transfer(
                "drain.wip",
                "inventory.wip",
                "inventory.drain",
                "commodity.grain",
                10,
            ),
        })
        .unwrap();

    let before = harness.state.clone();
    let before_hash = state_hash(&before).unwrap();

    let error = harness
        .apply(Command::AdvanceTo {
            tick: SimTick::new(10),
        })
        .unwrap_err();

    assert!(matches!(
        error,
        ApplyError::NegativeInventoryBalance { .. }
    ));
    assert_eq!(before_hash, state_hash(&harness.state).unwrap());
    assert_eq!(before, harness.state);
    assert_eq!(harness.state.tick(), SimTick::ZERO);
    assert_eq!(harness.state.scheduler().active_event_count(), 1);
    assert!(harness.state.scheduler().fired_events().is_empty());
    assert_eq!(
        harness
            .state
            .production_batch(&ProductionBatchId::new("batch.atomic"))
            .unwrap()
            .status(),
        ProductionBatchStatus::InProgress
    );
}

#[test]
fn replay_and_snapshot_preserve_autonomous_economy_state() {
    let commands = vec![
        CommandEnvelope::new(
            1,
            Command::OpenInventoryAccount {
                account_id: InventoryAccountId::new("inventory.food"),
                kind: InventoryAccountKind::Holding,
            },
        ),
        CommandEnvelope::new(
            2,
            Command::OpenInventoryAccount {
                account_id: InventoryAccountId::new("system.consumption"),
                kind: InventoryAccountKind::SourceOrSink,
            },
        ),
        CommandEnvelope::new(
            3,
            Command::OpenInventoryAccount {
                account_id: InventoryAccountId::new("system.seed"),
                kind: InventoryAccountKind::SourceOrSink,
            },
        ),
        CommandEnvelope::new(
            4,
            Command::ApplyTransaction {
                transaction: inventory_transfer(
                    "seed.replay.food",
                    "system.seed",
                    "inventory.food",
                    "commodity.grain",
                    100,
                ),
            },
        ),
        CommandEnvelope::new(
            5,
            Command::RegisterPopulationCohort {
                cohort: food_cohort(),
            },
        ),
        CommandEnvelope::new(
            6,
            Command::AdvanceTo {
                tick: SimTick::new(30),
            },
        ),
    ];

    let initial = WorldState::new(777);
    let first = replay(&initial, &commands).unwrap();
    let second = replay(&initial, &commands).unwrap();

    assert_eq!(state_hash(&first).unwrap(), state_hash(&second).unwrap());
    assert_eq!(first, second);
    assert_eq!(first.consumption_records().len(), 3);

    let encoded = encode_snapshot(&first).unwrap();
    let restored = decode_snapshot(&encoded).unwrap();

    assert_eq!(first, restored);
    assert_eq!(state_hash(&first).unwrap(), state_hash(&restored).unwrap());
}
