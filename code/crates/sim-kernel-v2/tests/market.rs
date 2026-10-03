use sim_kernel_v2::{
    decode_snapshot, derive_market_quote, encode_snapshot, execution_price, replay, state_hash,
    ApplyError, Command, CommandEnvelope, CommodityId, EconomicTransaction, EventId,
    InventoryAccountId, InventoryAccountKind, InventoryPosting, MarketId, MarketListing,
    MarketSide, MarketTradeId, MoneyAccountId, MoneyAccountKind, MoneyCp, MoneyPosting,
    PopulationCohort, PopulationCohortId, Quantity, SimTick, TransactionId, UnitPrice,
    WorldReducer, WorldState,
};

struct Harness {
    state: WorldState,
    sequence: u64,
}

impl Harness {
    fn new(market_grain: i64, actor_grain: i64, market_cash: i128, actor_cash: i128) -> Self {
        let mut harness = Self {
            state: WorldState::new(77),
            sequence: 0,
        };

        for (id, kind) in [
            ("inventory.market", InventoryAccountKind::Holding),
            ("inventory.actor", InventoryAccountKind::Holding),
            ("system.consumption", InventoryAccountKind::SourceOrSink),
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
            ("money.actor", MoneyAccountKind::Holding),
            ("money.seed", MoneyAccountKind::External),
        ] {
            harness
                .apply(Command::OpenMoneyAccount {
                    account_id: MoneyAccountId::new(id),
                    kind,
                })
                .unwrap();
        }

        if market_grain > 0 {
            harness
                .apply(Command::ApplyTransaction {
                    transaction: inventory_transfer(
                        "seed.market.grain",
                        "system.seed",
                        "inventory.market",
                        market_grain,
                    ),
                })
                .unwrap();
        }

        if actor_grain > 0 {
            harness
                .apply(Command::ApplyTransaction {
                    transaction: inventory_transfer(
                        "seed.actor.grain",
                        "system.seed",
                        "inventory.actor",
                        actor_grain,
                    ),
                })
                .unwrap();
        }

        if market_cash > 0 {
            harness
                .apply(Command::ApplyTransaction {
                    transaction: money_transfer(
                        "seed.market.cash",
                        "money.seed",
                        "money.market",
                        market_cash,
                    ),
                })
                .unwrap();
        }

        if actor_cash > 0 {
            harness
                .apply(Command::ApplyTransaction {
                    transaction: money_transfer(
                        "seed.actor.cash",
                        "money.seed",
                        "money.actor",
                        actor_cash,
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

    fn register_listing(&mut self, reference_milli_cp: i128, target: i64, depth: i64, window: u64) {
        self.apply(Command::RegisterMarketListing {
            listing: listing(reference_milli_cp, target, depth, window),
        })
        .unwrap();
    }
}

fn grain() -> CommodityId {
    CommodityId::new("commodity.grain")
}

fn market_id() -> MarketId {
    MarketId::new("market.waterdeep")
}

fn listing(
    reference_milli_cp: i128,
    target_stock: i64,
    depth: i64,
    demand_window_ticks: u64,
) -> MarketListing {
    MarketListing::new(
        market_id(),
        grain(),
        InventoryAccountId::new("inventory.market"),
        MoneyAccountId::new("money.market"),
        UnitPrice::from_milli_cp(reference_milli_cp),
        Quantity::new(target_stock),
        Quantity::new(depth),
        100,
        demand_window_ticks,
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

fn execute_buy(trade_id: &str, quantity: i64) -> Command {
    Command::ExecuteMarketTrade {
        trade_id: MarketTradeId::new(trade_id),
        market_id: market_id(),
        commodity_id: grain(),
        side: MarketSide::Buy,
        quantity: Quantity::new(quantity),
        actor_inventory_account: InventoryAccountId::new("inventory.actor"),
        actor_money_account: MoneyAccountId::new("money.actor"),
    }
}

fn execute_sell(trade_id: &str, quantity: i64) -> Command {
    Command::ExecuteMarketTrade {
        trade_id: MarketTradeId::new(trade_id),
        market_id: market_id(),
        commodity_id: grain(),
        side: MarketSide::Sell,
        quantity: Quantity::new(quantity),
        actor_inventory_account: InventoryAccountId::new("inventory.actor"),
        actor_money_account: MoneyAccountId::new("money.actor"),
    }
}

#[test]
fn low_price_goods_keep_nonzero_bid_ask_spread() {
    let mut harness = Harness::new(100, 0, 10_000, 10_000);
    harness.register_listing(1, 100, 100, 10);

    let quote = derive_market_quote(
        &harness.state,
        harness
            .state
            .market_listing(&market_id(), &grain())
            .unwrap(),
    )
    .unwrap();

    assert!(quote.bid.scaled_value() > 0);
    assert!(quote.ask.scaled_value() > quote.bid.scaled_value());
}

#[test]
fn scarcity_raises_fundamental_price_without_mutating_trade_history() {
    let mut harness = Harness::new(1_000, 0, 100_000, 100_000);
    harness.register_listing(2_000, 1_000, 1_000, 10);

    let listing = harness
        .state
        .market_listing(&market_id(), &grain())
        .unwrap()
        .clone();

    let normal = derive_market_quote(&harness.state, &listing).unwrap();
    assert!(harness.state.market_trades().is_empty());

    harness
        .apply(Command::ApplyTransaction {
            transaction: inventory_transfer(
                "move.market.stock",
                "inventory.market",
                "inventory.actor",
                900,
            ),
        })
        .unwrap();

    let scarce = derive_market_quote(&harness.state, &listing).unwrap();

    assert!(scarce.fundamental.scaled_value() > normal.fundamental.scaled_value());
    assert!(scarce.explanation.scarcity_factor_ppm > normal.explanation.scarcity_factor_ppm);
    assert!(harness.state.market_trades().is_empty());
}

#[test]
fn unmet_demand_pressure_expires_outside_configured_window() {
    let mut harness = Harness::new(0, 0, 100_000, 100_000);
    harness.register_listing(2_000, 100, 1_000, 15);

    harness
        .apply(Command::RegisterPopulationCohort {
            cohort: PopulationCohort::new(
                PopulationCohortId::new("cohort.market"),
                1_000,
                InventoryAccountId::new("inventory.market"),
                InventoryAccountId::new("system.consumption"),
                grain(),
                Quantity::new(60),
                10,
            ),
        })
        .unwrap();

    harness
        .apply(Command::AdvanceTo {
            tick: SimTick::new(10),
        })
        .unwrap();

    let listing = harness
        .state
        .market_listing(&market_id(), &grain())
        .unwrap()
        .clone();
    let pressured = derive_market_quote(&harness.state, &listing).unwrap();

    assert_eq!(pressured.explanation.recent_requested, Quantity::new(60));
    assert_eq!(pressured.explanation.recent_unmet, Quantity::new(60));
    assert!(pressured.explanation.demand_pressure_ppm > 1_000_000);

    harness
        .apply(Command::CancelEvent {
            event_id: EventId::new("population.consume.cohort.market"),
        })
        .unwrap();
    harness
        .apply(Command::AdvanceTo {
            tick: SimTick::new(30),
        })
        .unwrap();

    let expired = derive_market_quote(&harness.state, &listing).unwrap();

    assert_eq!(expired.explanation.recent_requested, Quantity::ZERO);
    assert_eq!(expired.explanation.recent_unmet, Quantity::ZERO);
    assert_eq!(expired.explanation.demand_pressure_ppm, 1_000_000);
    assert!(expired.fundamental.scaled_value() < pressured.fundamental.scaled_value());
}

#[test]
fn larger_buy_has_more_deterministic_market_impact() {
    let mut harness = Harness::new(1_000, 0, 100_000, 100_000);
    harness.register_listing(2_000, 1_000, 1_000, 10);

    let listing = harness
        .state
        .market_listing(&market_id(), &grain())
        .unwrap();
    let quote = derive_market_quote(&harness.state, listing).unwrap();

    let small =
        execution_price(&quote, MarketSide::Buy, Quantity::new(100), listing.depth()).unwrap();
    let large =
        execution_price(&quote, MarketSide::Buy, Quantity::new(500), listing.depth()).unwrap();

    assert!(large.scaled_value() > small.scaled_value());
}

#[test]
fn sell_trade_moves_goods_and_money_atomically_and_records_real_execution() {
    let mut harness = Harness::new(1_000, 200, 100_000, 10_000);
    harness.register_listing(2_000, 1_000, 1_000, 10);

    let listing = harness
        .state
        .market_listing(&market_id(), &grain())
        .unwrap()
        .clone();
    let quote = derive_market_quote(&harness.state, &listing).unwrap();
    let expected_price = execution_price(
        &quote,
        MarketSide::Sell,
        Quantity::new(100),
        listing.depth(),
    )
    .unwrap();
    let expected_value = expected_price.total_for(Quantity::new(100)).unwrap();

    let market_cash_before = harness
        .state
        .money_balance(&MoneyAccountId::new("money.market"))
        .unwrap();
    let actor_cash_before = harness
        .state
        .money_balance(&MoneyAccountId::new("money.actor"))
        .unwrap();

    harness.apply(execute_sell("trade.sell.001", 100)).unwrap();

    assert_eq!(
        harness
            .state
            .inventory_balance(&InventoryAccountId::new("inventory.market"), &grain()),
        Some(Quantity::new(1_100))
    );
    assert_eq!(
        harness
            .state
            .inventory_balance(&InventoryAccountId::new("inventory.actor"), &grain()),
        Some(Quantity::new(100))
    );

    let trade = &harness.state.market_trades()[0];
    assert_eq!(trade.side(), MarketSide::Sell);
    assert_eq!(trade.average_unit_price(), expected_price);
    assert_eq!(trade.total_value(), expected_value);

    assert_eq!(
        harness
            .state
            .money_balance(&MoneyAccountId::new("money.market"))
            .unwrap(),
        MoneyCp::new(market_cash_before.get() - expected_value.get())
    );
    assert_eq!(
        harness
            .state
            .money_balance(&MoneyAccountId::new("money.actor"))
            .unwrap(),
        MoneyCp::new(actor_cash_before.get() + expected_value.get())
    );
}

#[test]
fn buy_trade_moves_goods_and_money_atomically_and_records_real_execution() {
    let mut harness = Harness::new(1_000, 0, 100_000, 100_000);
    harness.register_listing(2_000, 1_000, 1_000, 10);

    let market_cash_before = harness
        .state
        .money_balance(&MoneyAccountId::new("money.market"))
        .unwrap();
    let actor_cash_before = harness
        .state
        .money_balance(&MoneyAccountId::new("money.actor"))
        .unwrap();

    harness.apply(execute_buy("trade.buy.001", 100)).unwrap();

    assert_eq!(
        harness
            .state
            .inventory_balance(&InventoryAccountId::new("inventory.market"), &grain()),
        Some(Quantity::new(900))
    );
    assert_eq!(
        harness
            .state
            .inventory_balance(&InventoryAccountId::new("inventory.actor"), &grain()),
        Some(Quantity::new(100))
    );

    assert_eq!(harness.state.market_trades().len(), 1);
    let trade = &harness.state.market_trades()[0];
    assert_eq!(trade.trade_id(), &MarketTradeId::new("trade.buy.001"));
    assert_eq!(trade.quantity(), Quantity::new(100));
    assert!(trade.total_value().get() > 0);

    assert_eq!(
        harness
            .state
            .money_balance(&MoneyAccountId::new("money.market"))
            .unwrap(),
        MoneyCp::new(market_cash_before.get() + trade.total_value().get())
    );
    assert_eq!(
        harness
            .state
            .money_balance(&MoneyAccountId::new("money.actor"))
            .unwrap(),
        MoneyCp::new(actor_cash_before.get() - trade.total_value().get())
    );
}

#[test]
fn insufficient_cash_or_inventory_rejects_without_partial_state() {
    let mut cash_limited = Harness::new(1_000, 0, 100_000, 0);
    cash_limited.register_listing(2_000, 1_000, 1_000, 10);

    let before_cash = state_hash(&cash_limited.state).unwrap();
    let error = cash_limited
        .apply(execute_buy("trade.no.cash", 100))
        .unwrap_err();
    assert!(matches!(error, ApplyError::NegativeMoneyBalance { .. }));
    assert_eq!(before_cash, state_hash(&cash_limited.state).unwrap());
    assert!(cash_limited.state.market_trades().is_empty());

    let mut inventory_limited = Harness::new(1_000, 0, 100_000, 100_000);
    inventory_limited.register_listing(2_000, 1_000, 1_000, 10);

    let before_inventory = state_hash(&inventory_limited.state).unwrap();
    let error = inventory_limited
        .apply(execute_sell("trade.no.inventory", 100))
        .unwrap_err();
    assert!(matches!(error, ApplyError::NegativeInventoryBalance { .. }));
    assert_eq!(
        before_inventory,
        state_hash(&inventory_limited.state).unwrap()
    );
    assert!(inventory_limited.state.market_trades().is_empty());
}

#[test]
fn invalid_listing_configuration_is_rejected() {
    let cases = [
        (
            MarketListing::new(
                market_id(),
                grain(),
                InventoryAccountId::new("inventory.market"),
                MoneyAccountId::new("money.market"),
                UnitPrice::new(0, 1_000).unwrap(),
                Quantity::new(100),
                Quantity::new(100),
                100,
                10,
            ),
            ApplyError::InvalidMarketReferencePrice,
        ),
        (
            listing(2_000, 0, 100, 10),
            ApplyError::InvalidMarketTargetStock,
        ),
        (listing(2_000, 100, 0, 10), ApplyError::InvalidMarketDepth),
        (
            MarketListing::new(
                market_id(),
                grain(),
                InventoryAccountId::new("inventory.market"),
                MoneyAccountId::new("money.market"),
                UnitPrice::from_milli_cp(2_000),
                Quantity::new(100),
                Quantity::new(100),
                0,
                10,
            ),
            ApplyError::InvalidMarketSpread,
        ),
        (
            listing(2_000, 100, 100, 0),
            ApplyError::InvalidMarketDemandWindow,
        ),
    ];

    for (candidate, expected) in cases {
        let mut harness = Harness::new(100, 0, 100_000, 100_000);
        let before = state_hash(&harness.state).unwrap();

        let error = harness
            .apply(Command::RegisterMarketListing { listing: candidate })
            .unwrap_err();

        assert_eq!(error, expected);
        assert_eq!(before, state_hash(&harness.state).unwrap());
        assert!(harness
            .state
            .market_listing(&market_id(), &grain())
            .is_none());
    }
}

#[test]
fn replay_and_snapshot_preserve_market_listing_and_trade_history() {
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
                account_id: InventoryAccountId::new("inventory.actor"),
                kind: InventoryAccountKind::Holding,
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
            Command::OpenMoneyAccount {
                account_id: MoneyAccountId::new("money.market"),
                kind: MoneyAccountKind::Holding,
            },
        ),
        CommandEnvelope::new(
            5,
            Command::OpenMoneyAccount {
                account_id: MoneyAccountId::new("money.actor"),
                kind: MoneyAccountKind::Holding,
            },
        ),
        CommandEnvelope::new(
            6,
            Command::OpenMoneyAccount {
                account_id: MoneyAccountId::new("money.seed"),
                kind: MoneyAccountKind::External,
            },
        ),
        CommandEnvelope::new(
            7,
            Command::ApplyTransaction {
                transaction: inventory_transfer(
                    "replay.seed.market.grain",
                    "system.seed",
                    "inventory.market",
                    1_000,
                ),
            },
        ),
        CommandEnvelope::new(
            8,
            Command::ApplyTransaction {
                transaction: money_transfer(
                    "replay.seed.market.cash",
                    "money.seed",
                    "money.market",
                    100_000,
                ),
            },
        ),
        CommandEnvelope::new(
            9,
            Command::ApplyTransaction {
                transaction: money_transfer(
                    "replay.seed.actor.cash",
                    "money.seed",
                    "money.actor",
                    100_000,
                ),
            },
        ),
        CommandEnvelope::new(
            10,
            Command::RegisterMarketListing {
                listing: listing(2_000, 1_000, 1_000, 10),
            },
        ),
        CommandEnvelope::new(11, execute_buy("trade.replay.001", 100)),
    ];

    let initial = WorldState::new(88);
    let first = replay(&initial, &commands).unwrap();
    let second = replay(&initial, &commands).unwrap();

    assert_eq!(first, second);
    assert_eq!(state_hash(&first).unwrap(), state_hash(&second).unwrap());
    assert_eq!(first.market_trades().len(), 1);

    let encoded = encode_snapshot(&first).unwrap();
    let restored = decode_snapshot(&encoded).unwrap();

    assert_eq!(first, restored);
    assert_eq!(state_hash(&first).unwrap(), state_hash(&restored).unwrap());
}
