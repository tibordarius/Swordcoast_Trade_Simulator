use sim_kernel_v2::{
    Command, CommandEnvelope, EconomicTransaction, InventoryAccountId, InventoryAccountKind,
    InventoryPosting, MoneyAccountId, MoneyAccountKind, MoneyCp, MoneyPosting, Quantity,
    TransactionId,
};

use crate::constants::{
    OPENING_INVENTORY_ACCOUNT_ID, OPENING_INVENTORY_TRANSACTION_ID, OPENING_MONEY_ACCOUNT_ID,
    OPENING_MONEY_TRANSACTION_ID,
};
use crate::ValidatedScenarioPack;

pub fn compile_initialization_commands(pack: &ValidatedScenarioPack) -> Vec<CommandEnvelope> {
    let mut commands = Vec::new();
    let mut sequence = 0_u64;

    let has_opening_inventory = pack
        .pack()
        .opening_inventory
        .iter()
        .any(|row| row.quantity != 0);
    let has_opening_money = pack
        .pack()
        .opening_money
        .iter()
        .any(|row| row.amount_cp != 0);

    if has_opening_inventory {
        sequence += 1;
        commands.push(CommandEnvelope::new(
            sequence,
            Command::OpenInventoryAccount {
                account_id: InventoryAccountId::new(OPENING_INVENTORY_ACCOUNT_ID),
                kind: InventoryAccountKind::SourceOrSink,
            },
        ));
    }

    let mut inventory_accounts: Vec<_> = pack.pack().inventory_accounts.iter().collect();
    inventory_accounts.sort_by(|a, b| a.id.cmp(&b.id));

    for account in inventory_accounts {
        sequence += 1;
        commands.push(CommandEnvelope::new(
            sequence,
            Command::OpenInventoryAccount {
                account_id: account.id.clone(),
                kind: account.kind,
            },
        ));
    }

    if has_opening_money {
        sequence += 1;
        commands.push(CommandEnvelope::new(
            sequence,
            Command::OpenMoneyAccount {
                account_id: MoneyAccountId::new(OPENING_MONEY_ACCOUNT_ID),
                kind: MoneyAccountKind::External,
            },
        ));
    }

    let mut money_accounts: Vec<_> = pack.pack().money_accounts.iter().collect();
    money_accounts.sort_by(|a, b| a.id.cmp(&b.id));

    for account in money_accounts {
        sequence += 1;
        commands.push(CommandEnvelope::new(
            sequence,
            Command::OpenMoneyAccount {
                account_id: account.id.clone(),
                kind: account.kind,
            },
        ));
    }

    let mut opening_inventory: Vec<_> = pack
        .pack()
        .opening_inventory
        .iter()
        .filter(|row| row.quantity != 0)
        .collect();
    opening_inventory.sort_by(|a, b| {
        a.account_id
            .cmp(&b.account_id)
            .then_with(|| a.commodity_id.cmp(&b.commodity_id))
    });

    if !opening_inventory.is_empty() {
        let source_account = InventoryAccountId::new(OPENING_INVENTORY_ACCOUNT_ID);
        let mut postings = Vec::with_capacity(opening_inventory.len() * 2);

        for row in opening_inventory {
            postings.push(InventoryPosting::new(
                source_account.clone(),
                row.commodity_id.clone(),
                Quantity::new(-row.quantity),
            ));
            postings.push(InventoryPosting::new(
                row.account_id.clone(),
                row.commodity_id.clone(),
                Quantity::new(row.quantity),
            ));
        }

        sequence += 1;
        commands.push(CommandEnvelope::new(
            sequence,
            Command::ApplyTransaction {
                transaction: EconomicTransaction::new(
                    TransactionId::new(OPENING_INVENTORY_TRANSACTION_ID),
                    postings,
                    vec![],
                ),
            },
        ));
    }

    let mut opening_money: Vec<_> = pack
        .pack()
        .opening_money
        .iter()
        .filter(|row| row.amount_cp != 0)
        .collect();
    opening_money.sort_by(|a, b| a.account_id.cmp(&b.account_id));

    if !opening_money.is_empty() {
        let external_account = MoneyAccountId::new(OPENING_MONEY_ACCOUNT_ID);
        let mut postings = Vec::with_capacity(opening_money.len() + 1);
        let total: i128 = opening_money
            .iter()
            .map(|row| i128::from(row.amount_cp))
            .sum();

        postings.push(MoneyPosting::new(external_account, MoneyCp::new(-total)));

        for row in opening_money {
            postings.push(MoneyPosting::new(
                row.account_id.clone(),
                MoneyCp::new(i128::from(row.amount_cp)),
            ));
        }

        sequence += 1;
        commands.push(CommandEnvelope::new(
            sequence,
            Command::ApplyTransaction {
                transaction: EconomicTransaction::new(
                    TransactionId::new(OPENING_MONEY_TRANSACTION_ID),
                    vec![],
                    postings,
                ),
            },
        ));
    }

    commands
}
