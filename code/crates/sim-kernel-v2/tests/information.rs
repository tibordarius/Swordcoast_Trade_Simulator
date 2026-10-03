use sim_kernel_v2::{
    decode_snapshot, derive_market_quote, encode_snapshot, replay, state_hash, ActorId, ApplyError,
    Command, CommandEnvelope, CommodityId, EconomicTransaction, InventoryAccountId,
    InventoryAccountKind, InventoryPosting, MarketId, MarketListing, MarketObservationId,
    MoneyAccountId, MoneyAccountKind, MoneyCp, MoneyPosting, Quantity, SimTick, TransactionId,
    UnitPrice, WorldReducer, WorldState,
};

struct Harness {
    state: WorldState,
    sequence: u64,
}

impl Harness {
    fn new(stock: i64) -> Self {
        let mut harness = Self {
            state: WorldState::new(5150),
            sequence: 0,
        };

        for (id, kind) in [
            ("inventory.market", InventoryAccountKind::Holding),
            ("inventory.buffer", InventoryAccountKind::Holding),
            ("system.seed", InventoryAccountKind::SourceOrSink),
        ] {
            harness
                .apply(Command::OpenInventoryAccount {
                    account_id: InventoryAccountId::new(id),
                    kind,
                })
                .unwrap();
        }

        for (id, kind) in [
            ("money.market", MoneyAccountKind::Holding),
            ("money.seed", MoneyAccountKind::External),
        ] {
            harness
                .apply(Command::OpenMoneyAccount {
                    account_id: MoneyAccountId::new(id),
                    kind,
                })
                .unwrap();
        }

        if stock > 0 {
            harness
                .apply(Command::ApplyTransaction {
                    transaction: inventory_transfer(
                        "seed.market.grain",
                        "system.seed",
                        "inventory.market",
                        stock,
                    ),
                })
                .unwrap();
        }

        harness
            .apply(Command::ApplyTransaction {
                transaction: money_transfer(
                    "seed.market.money",
                    "money.seed",
                    "money.market",
                    100_000,
                ),
            })
            .unwrap();

        harness
            .apply(Command::RegisterMarketListing { listing: listing() })
            .unwrap();

        harness
    }

    fn apply(&mut self, command: Command) -> Result<(), ApplyError> {
        self.sequence += 1;
        WorldReducer::apply(
            &mut self.state,
            &CommandEnvelope::new(self.sequence, command),
        )
    }

    fn register_actor(&mut self, actor: &str) {
        self.apply(Command::RegisterKnowledgeActor {
            actor_id: ActorId::new(actor),
        })
        .unwrap();
    }

    fn dispatch(
        &mut self,
        observation_id: &str,
        actor: &str,
        delay_ticks: u64,
    ) -> Result<(), ApplyError> {
        self.apply(Command::DispatchMarketObservation {
            observation_id: MarketObservationId::new(observation_id),
            actor_id: ActorId::new(actor),
            market_id: market_id(),
            commodity_id: grain(),
            delay_ticks,
        })
    }
}

fn grain() -> CommodityId {
    CommodityId::new("commodity.grain")
}

fn market_id() -> MarketId {
    MarketId::new("market.waterdeep")
}

fn listing() -> MarketListing {
    MarketListing::new(
        market_id(),
        grain(),
        InventoryAccountId::new("inventory.market"),
        MoneyAccountId::new("money.market"),
        UnitPrice::from_milli_cp(2_000),
        Quantity::new(1_000),
        Quantity::new(1_000),
        100,
        20,
    )
}

fn inventory_transfer(
    transaction_id: &str,
    from: &str,
    to: &str,
    quantity: i64,
) -> EconomicTransaction {
    EconomicTransaction::new(
        TransactionId::new(transaction_id),
        vec![
            InventoryPosting::new(
                InventoryAccountId::new(from),
                grain(),
                Quantity::new(-quantity),
            ),
            InventoryPosting::new(
                InventoryAccountId::new(to),
                grain(),
                Quantity::new(quantity),
            ),
        ],
        vec![],
    )
}

fn money_transfer(transaction_id: &str, from: &str, to: &str, amount: i128) -> EconomicTransaction {
    EconomicTransaction::new(
        TransactionId::new(transaction_id),
        vec![],
        vec![
            MoneyPosting::new(MoneyAccountId::new(from), MoneyCp::new(-amount)),
            MoneyPosting::new(MoneyAccountId::new(to), MoneyCp::new(amount)),
        ],
    )
}

#[test]
fn observation_is_invisible_until_scheduled_delivery() {
    let mut harness = Harness::new(1_000);
    harness.register_actor("merchant.1");
    harness.dispatch("obs.1", "merchant.1", 10).unwrap();

    let before = harness
        .state
        .knowledge_view(&ActorId::new("merchant.1"))
        .unwrap();
    assert!(before.market_observations().is_empty());
    assert!(harness
        .state
        .has_market_observation(&MarketObservationId::new("obs.1")));

    harness
        .apply(Command::AdvanceTo {
            tick: SimTick::new(9),
        })
        .unwrap();

    assert!(harness
        .state
        .knowledge_view(&ActorId::new("merchant.1"))
        .unwrap()
        .market_observations()
        .is_empty());

    harness
        .apply(Command::AdvanceTo {
            tick: SimTick::new(10),
        })
        .unwrap();

    let view = harness
        .state
        .knowledge_view(&ActorId::new("merchant.1"))
        .unwrap();
    let observation = view
        .latest_market_observation(&market_id(), &grain())
        .unwrap();

    assert_eq!(
        observation.observation_id(),
        &MarketObservationId::new("obs.1")
    );
    assert_eq!(observation.observed_at(), SimTick::ZERO);
    assert_eq!(observation.delivery_tick(), SimTick::new(10));
    assert_eq!(observation.transport_delay_ticks(), 10);
    assert_eq!(view.observation_age_ticks(&market_id(), &grain()), Some(10));
}

#[test]
fn in_flight_observation_keeps_dispatch_time_quote_when_market_changes() {
    let mut harness = Harness::new(1_000);
    harness.register_actor("merchant.1");

    let truth_at_dispatch = derive_market_quote(
        &harness.state,
        harness
            .state
            .market_listing(&market_id(), &grain())
            .unwrap(),
    )
    .unwrap();

    harness.dispatch("obs.old", "merchant.1", 10).unwrap();

    harness
        .apply(Command::ApplyTransaction {
            transaction: inventory_transfer(
                "drain.market",
                "inventory.market",
                "inventory.buffer",
                900,
            ),
        })
        .unwrap();

    harness
        .apply(Command::AdvanceTo {
            tick: SimTick::new(10),
        })
        .unwrap();

    let current_truth = derive_market_quote(
        &harness.state,
        harness
            .state
            .market_listing(&market_id(), &grain())
            .unwrap(),
    )
    .unwrap();
    let view = harness
        .state
        .knowledge_view(&ActorId::new("merchant.1"))
        .unwrap();
    let observed = view
        .latest_market_observation(&market_id(), &grain())
        .unwrap();

    assert_eq!(observed.quote(), &truth_at_dispatch);
    assert!(current_truth.fundamental.scaled_value() > observed.quote().fundamental.scaled_value());
    assert!(harness.state.market_trades().is_empty());
}

#[test]
fn late_older_observation_does_not_overwrite_newer_observed_truth() {
    let mut harness = Harness::new(1_000);
    harness.register_actor("merchant.1");

    harness.dispatch("obs.old.slow", "merchant.1", 20).unwrap();

    harness
        .apply(Command::AdvanceTo {
            tick: SimTick::new(5),
        })
        .unwrap();
    harness
        .apply(Command::ApplyTransaction {
            transaction: inventory_transfer(
                "drain.before.new",
                "inventory.market",
                "inventory.buffer",
                900,
            ),
        })
        .unwrap();

    harness.dispatch("obs.new.fast", "merchant.1", 5).unwrap();

    harness
        .apply(Command::AdvanceTo {
            tick: SimTick::new(10),
        })
        .unwrap();

    let first_view = harness
        .state
        .knowledge_view(&ActorId::new("merchant.1"))
        .unwrap();
    assert_eq!(
        first_view
            .latest_market_observation(&market_id(), &grain())
            .unwrap()
            .observation_id(),
        &MarketObservationId::new("obs.new.fast")
    );

    harness
        .apply(Command::AdvanceTo {
            tick: SimTick::new(20),
        })
        .unwrap();

    let final_view = harness
        .state
        .knowledge_view(&ActorId::new("merchant.1"))
        .unwrap();
    let latest = final_view
        .latest_market_observation(&market_id(), &grain())
        .unwrap();

    assert_eq!(
        latest.observation_id(),
        &MarketObservationId::new("obs.new.fast")
    );
    assert_eq!(latest.observed_at(), SimTick::new(5));

    let history: Vec<&str> = harness
        .state
        .delivered_market_observations()
        .iter()
        .map(|observation| observation.observation_id().as_str())
        .collect();
    assert_eq!(history, vec!["obs.new.fast", "obs.old.slow"]);
}

#[test]
fn actor_views_are_isolated() {
    let mut harness = Harness::new(1_000);
    harness.register_actor("merchant.a");
    harness.register_actor("merchant.b");

    harness.dispatch("obs.a", "merchant.a", 5).unwrap();
    harness
        .apply(Command::AdvanceTo {
            tick: SimTick::new(5),
        })
        .unwrap();

    let a = harness
        .state
        .knowledge_view(&ActorId::new("merchant.a"))
        .unwrap();
    let b = harness
        .state
        .knowledge_view(&ActorId::new("merchant.b"))
        .unwrap();

    assert!(a
        .latest_market_observation(&market_id(), &grain())
        .is_some());
    assert!(b.market_observations().is_empty());
}

#[test]
fn unknown_actor_unknown_market_and_duplicate_observation_are_rejected_atomically() {
    let mut harness = Harness::new(1_000);

    let before_unknown_actor = state_hash(&harness.state).unwrap();
    let error = harness
        .dispatch("obs.unknown", "merchant.missing", 5)
        .unwrap_err();
    assert_eq!(
        error,
        ApplyError::UnknownKnowledgeActor(ActorId::new("merchant.missing"))
    );
    assert_eq!(before_unknown_actor, state_hash(&harness.state).unwrap());

    harness.register_actor("merchant.1");

    let before_unknown_market = state_hash(&harness.state).unwrap();
    let error = harness
        .apply(Command::DispatchMarketObservation {
            observation_id: MarketObservationId::new("obs.missing.market"),
            actor_id: ActorId::new("merchant.1"),
            market_id: MarketId::new("market.missing"),
            commodity_id: grain(),
            delay_ticks: 5,
        })
        .unwrap_err();
    assert!(matches!(error, ApplyError::UnknownMarketListing { .. }));
    assert_eq!(before_unknown_market, state_hash(&harness.state).unwrap());

    harness.dispatch("obs.dup", "merchant.1", 5).unwrap();
    let before_duplicate = state_hash(&harness.state).unwrap();
    let error = harness.dispatch("obs.dup", "merchant.1", 10).unwrap_err();
    assert_eq!(
        error,
        ApplyError::DuplicateMarketObservation(MarketObservationId::new("obs.dup"))
    );
    assert_eq!(before_duplicate, state_hash(&harness.state).unwrap());
}

#[test]
fn information_delivery_does_not_change_market_balances_or_trade_history() {
    let mut harness = Harness::new(1_000);
    harness.register_actor("merchant.1");

    let inventory_before = harness
        .state
        .inventory_balance(&InventoryAccountId::new("inventory.market"), &grain())
        .unwrap();
    let money_before = harness
        .state
        .money_balance(&MoneyAccountId::new("money.market"))
        .unwrap();

    harness.dispatch("obs.readonly", "merchant.1", 10).unwrap();
    harness
        .apply(Command::AdvanceTo {
            tick: SimTick::new(10),
        })
        .unwrap();

    assert_eq!(
        harness
            .state
            .inventory_balance(&InventoryAccountId::new("inventory.market"), &grain())
            .unwrap(),
        inventory_before
    );
    assert_eq!(
        harness
            .state
            .money_balance(&MoneyAccountId::new("money.market"))
            .unwrap(),
        money_before
    );
    assert!(harness.state.market_trades().is_empty());
}

#[test]
fn pending_observation_survives_snapshot_and_delivers_identically() {
    let mut harness = Harness::new(1_000);
    harness.register_actor("merchant.1");
    harness.dispatch("obs.pending", "merchant.1", 10).unwrap();

    let encoded = encode_snapshot(&harness.state).unwrap();
    let mut restored = decode_snapshot(&encoded).unwrap();

    assert_eq!(
        state_hash(&harness.state).unwrap(),
        state_hash(&restored).unwrap()
    );
    assert!(restored
        .knowledge_view(&ActorId::new("merchant.1"))
        .unwrap()
        .market_observations()
        .is_empty());

    let next_sequence = harness.sequence + 1;
    let advance = CommandEnvelope::new(
        next_sequence,
        Command::AdvanceTo {
            tick: SimTick::new(10),
        },
    );

    WorldReducer::apply(&mut harness.state, &advance).unwrap();
    WorldReducer::apply(&mut restored, &advance).unwrap();

    assert_eq!(harness.state, restored);
    assert_eq!(
        harness
            .state
            .knowledge_view(&ActorId::new("merchant.1"))
            .unwrap()
            .latest_market_observation(&market_id(), &grain())
            .unwrap()
            .observation_id(),
        &MarketObservationId::new("obs.pending")
    );
}

#[test]
fn replay_reproduces_delayed_information_state() {
    let commands = vec![
        CommandEnvelope::new(
            1,
            Command::OpenInventoryAccount {
                account_id: InventoryAccountId::new("inventory.market"),
                kind: InventoryAccountKind::Holding,
            },
        ),
        CommandEnvelope::new(
            2,
            Command::OpenInventoryAccount {
                account_id: InventoryAccountId::new("system.seed"),
                kind: InventoryAccountKind::SourceOrSink,
            },
        ),
        CommandEnvelope::new(
            3,
            Command::OpenMoneyAccount {
                account_id: MoneyAccountId::new("money.market"),
                kind: MoneyAccountKind::Holding,
            },
        ),
        CommandEnvelope::new(
            4,
            Command::ApplyTransaction {
                transaction: inventory_transfer(
                    "replay.seed",
                    "system.seed",
                    "inventory.market",
                    1_000,
                ),
            },
        ),
        CommandEnvelope::new(5, Command::RegisterMarketListing { listing: listing() }),
        CommandEnvelope::new(
            6,
            Command::RegisterKnowledgeActor {
                actor_id: ActorId::new("merchant.1"),
            },
        ),
        CommandEnvelope::new(
            7,
            Command::DispatchMarketObservation {
                observation_id: MarketObservationId::new("obs.replay"),
                actor_id: ActorId::new("merchant.1"),
                market_id: market_id(),
                commodity_id: grain(),
                delay_ticks: 10,
            },
        ),
        CommandEnvelope::new(
            8,
            Command::AdvanceTo {
                tick: SimTick::new(10),
            },
        ),
    ];

    let initial = WorldState::new(5150);
    let first = replay(&initial, &commands).unwrap();
    let second = replay(&initial, &commands).unwrap();

    assert_eq!(first, second);
    assert_eq!(state_hash(&first).unwrap(), state_hash(&second).unwrap());
    assert_eq!(first.delivered_market_observations().len(), 1);
}
