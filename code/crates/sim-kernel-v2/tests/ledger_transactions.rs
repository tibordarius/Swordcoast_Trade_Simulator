use sim_kernel_v2::{
    decode_snapshot, encode_snapshot, replay, state_hash, ApplyError, Command, CommandEnvelope,
    CommodityId, EconomicTransaction, InventoryAccountId, InventoryAccountKind, InventoryPosting,
    MoneyAccountId, MoneyAccountKind, MoneyCp, MoneyPosting, Quantity, TransactionId, WorldReducer,
    WorldState,
};

fn open_accounts(state: &mut WorldState) {
    let commands = [
        CommandEnvelope::new(
            1,
            Command::OpenInventoryAccount {
                account_id: InventoryAccountId::new("system.production"),
                kind: InventoryAccountKind::SourceOrSink,
            },
        ),
        CommandEnvelope::new(
            2,
            Command::OpenInventoryAccount {
                account_id: InventoryAccountId::new("warehouse.waterdeep"),
                kind: InventoryAccountKind::Holding,
            },
        ),
        CommandEnvelope::new(
            3,
            Command::OpenMoneyAccount {
                account_id: MoneyAccountId::new("external.money"),
                kind: MoneyAccountKind::External,
            },
        ),
        CommandEnvelope::new(
            4,
            Command::OpenMoneyAccount {
                account_id: MoneyAccountId::new("merchant.cash"),
                kind: MoneyAccountKind::Holding,
            },
        ),
    ];

    for command in &commands {
        WorldReducer::apply(state, command).unwrap();
    }
}

fn seed_transaction(id: &str) -> EconomicTransaction {
    EconomicTransaction::new(
        TransactionId::new(id),
        vec![
            InventoryPosting::new(
                InventoryAccountId::new("system.production"),
                CommodityId::new("grain"),
                Quantity::new(-100),
            ),
            InventoryPosting::new(
                InventoryAccountId::new("warehouse.waterdeep"),
                CommodityId::new("grain"),
                Quantity::new(100),
            ),
        ],
        vec![
            MoneyPosting::new(MoneyAccountId::new("external.money"), MoneyCp::new(-500)),
            MoneyPosting::new(MoneyAccountId::new("merchant.cash"), MoneyCp::new(500)),
        ],
    )
}

#[test]
fn balanced_transaction_updates_goods_money_and_ledgers_once() {
    let mut state = WorldState::new(42);
    open_accounts(&mut state);

    let transaction = seed_transaction("tx.seed.001");
    WorldReducer::apply(
        &mut state,
        &CommandEnvelope::new(5, Command::ApplyTransaction { transaction }),
    )
    .unwrap();

    let grain = CommodityId::new("grain");
    assert_eq!(
        state.inventory_balance(&InventoryAccountId::new("system.production"), &grain),
        Some(Quantity::new(-100))
    );
    assert_eq!(
        state.inventory_balance(&InventoryAccountId::new("warehouse.waterdeep"), &grain),
        Some(Quantity::new(100))
    );
    assert_eq!(
        state.money_balance(&MoneyAccountId::new("external.money")),
        Some(MoneyCp::new(-500))
    );
    assert_eq!(
        state.money_balance(&MoneyAccountId::new("merchant.cash")),
        Some(MoneyCp::new(500))
    );

    assert_eq!(state.inventory_ledger().len(), 2);
    assert_eq!(state.money_ledger().len(), 2);
    assert!(state.has_applied_transaction(&TransactionId::new("tx.seed.001")));
    assert_eq!(state.revision().get(), 5);
}

#[test]
fn invalid_mixed_transaction_is_atomic() {
    let mut state = WorldState::new(42);
    open_accounts(&mut state);
    let revision_before = state.revision();

    let transaction = EconomicTransaction::new(
        TransactionId::new("tx.invalid"),
        vec![
            InventoryPosting::new(
                InventoryAccountId::new("system.production"),
                CommodityId::new("grain"),
                Quantity::new(-10),
            ),
            InventoryPosting::new(
                InventoryAccountId::new("warehouse.waterdeep"),
                CommodityId::new("grain"),
                Quantity::new(10),
            ),
        ],
        vec![
            MoneyPosting::new(MoneyAccountId::new("external.money"), MoneyCp::new(-100)),
            MoneyPosting::new(MoneyAccountId::new("merchant.cash"), MoneyCp::new(200)),
        ],
    );

    let error = WorldReducer::apply(
        &mut state,
        &CommandEnvelope::new(5, Command::ApplyTransaction { transaction }),
    )
    .unwrap_err();

    assert_eq!(
        error,
        ApplyError::MoneyNotConserved {
            net: MoneyCp::new(100)
        }
    );

    let grain = CommodityId::new("grain");
    assert_eq!(
        state.inventory_balance(&InventoryAccountId::new("warehouse.waterdeep"), &grain),
        Some(Quantity::ZERO)
    );
    assert_eq!(
        state.money_balance(&MoneyAccountId::new("merchant.cash")),
        Some(MoneyCp::ZERO)
    );
    assert!(state.inventory_ledger().is_empty());
    assert!(state.money_ledger().is_empty());
    assert_eq!(state.revision(), revision_before);
}

#[test]
fn holding_account_cannot_go_negative() {
    let mut state = WorldState::new(42);
    open_accounts(&mut state);

    let transaction = EconomicTransaction::new(
        TransactionId::new("tx.negative"),
        vec![
            InventoryPosting::new(
                InventoryAccountId::new("warehouse.waterdeep"),
                CommodityId::new("grain"),
                Quantity::new(-1),
            ),
            InventoryPosting::new(
                InventoryAccountId::new("system.production"),
                CommodityId::new("grain"),
                Quantity::new(1),
            ),
        ],
        vec![],
    );

    let error = WorldReducer::apply(
        &mut state,
        &CommandEnvelope::new(5, Command::ApplyTransaction { transaction }),
    )
    .unwrap_err();

    assert_eq!(
        error,
        ApplyError::NegativeInventoryBalance {
            account_id: InventoryAccountId::new("warehouse.waterdeep"),
            commodity_id: CommodityId::new("grain"),
            attempted: Quantity::new(-1),
        }
    );
    assert!(state.inventory_ledger().is_empty());
}

#[test]
fn duplicate_transaction_id_is_rejected_without_second_commit() {
    let mut state = WorldState::new(42);
    open_accounts(&mut state);

    WorldReducer::apply(
        &mut state,
        &CommandEnvelope::new(
            5,
            Command::ApplyTransaction {
                transaction: seed_transaction("tx.seed.dup"),
            },
        ),
    )
    .unwrap();

    let revision_after_first = state.revision();
    let error = WorldReducer::apply(
        &mut state,
        &CommandEnvelope::new(
            6,
            Command::ApplyTransaction {
                transaction: seed_transaction("tx.seed.dup"),
            },
        ),
    )
    .unwrap_err();

    assert_eq!(
        error,
        ApplyError::DuplicateTransaction(TransactionId::new("tx.seed.dup"))
    );
    assert_eq!(state.revision(), revision_after_first);
    assert_eq!(state.inventory_ledger().len(), 2);
    assert_eq!(state.money_ledger().len(), 2);
}

#[test]
fn replay_and_snapshot_preserve_ledger_state() {
    let commands = vec![
        CommandEnvelope::new(
            1,
            Command::OpenInventoryAccount {
                account_id: InventoryAccountId::new("system.production"),
                kind: InventoryAccountKind::SourceOrSink,
            },
        ),
        CommandEnvelope::new(
            2,
            Command::OpenInventoryAccount {
                account_id: InventoryAccountId::new("warehouse.waterdeep"),
                kind: InventoryAccountKind::Holding,
            },
        ),
        CommandEnvelope::new(
            3,
            Command::OpenMoneyAccount {
                account_id: MoneyAccountId::new("external.money"),
                kind: MoneyAccountKind::External,
            },
        ),
        CommandEnvelope::new(
            4,
            Command::OpenMoneyAccount {
                account_id: MoneyAccountId::new("merchant.cash"),
                kind: MoneyAccountKind::Holding,
            },
        ),
        CommandEnvelope::new(
            5,
            Command::ApplyTransaction {
                transaction: seed_transaction("tx.replay.001"),
            },
        ),
    ];

    let initial = WorldState::new(999);
    let first = replay(&initial, &commands).unwrap();
    let second = replay(&initial, &commands).unwrap();

    assert_eq!(state_hash(&first).unwrap(), state_hash(&second).unwrap());

    let encoded = encode_snapshot(&first).unwrap();
    let restored = decode_snapshot(&encoded).unwrap();
    assert_eq!(state_hash(&first).unwrap(), state_hash(&restored).unwrap());
    assert_eq!(first, restored);
}
