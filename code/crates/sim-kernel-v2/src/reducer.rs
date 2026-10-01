use std::collections::BTreeMap;

use crate::{
    Command, CommandEnvelope, CommodityId, EconomicTransaction, EventDomain, EventId,
    InventoryAccount, InventoryAccountId, InventoryAccountKind, MoneyAccount, MoneyAccountId,
    MoneyAccountKind, MoneyCp, Quantity, SimTick, TransactionId, WorldRevision, WorldState,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ApplyError {
    TimeRegression {
        current: SimTick,
        requested: SimTick,
    },
    EventInPast {
        current: SimTick,
        requested: SimTick,
    },
    UnknownActiveEvent(EventId),
    EventSequenceOverflow,
    EventGenerationOverflow(EventId),
    RevisionOverflow,
    DuplicateInventoryAccount(InventoryAccountId),
    DuplicateMoneyAccount(MoneyAccountId),
    DuplicateTransaction(TransactionId),
    EmptyTransaction,
    UnknownInventoryAccount(InventoryAccountId),
    UnknownMoneyAccount(MoneyAccountId),
    InventoryNotConserved {
        commodity_id: CommodityId,
        net: Quantity,
    },
    MoneyNotConserved {
        net: MoneyCp,
    },
    NegativeInventoryBalance {
        account_id: InventoryAccountId,
        commodity_id: CommodityId,
        attempted: Quantity,
    },
    NegativeMoneyBalance {
        account_id: MoneyAccountId,
        attempted: MoneyCp,
    },
    ArithmeticOverflow,
}

pub struct WorldReducer;

impl WorldReducer {
    pub fn apply(state: &mut WorldState, envelope: &CommandEnvelope) -> Result<(), ApplyError> {
        match envelope.command() {
            Command::AdvanceTo { tick } => Self::advance_to(state, *tick),
            Command::ScheduleEvent {
                event_id,
                at_tick,
                domain,
            } => Self::schedule_event(state, event_id, *at_tick, *domain),
            Command::CancelEvent { event_id } => Self::cancel_event(state, event_id),
            Command::OpenInventoryAccount { account_id, kind } => {
                Self::open_inventory_account(state, account_id, *kind)
            }
            Command::OpenMoneyAccount { account_id, kind } => {
                Self::open_money_account(state, account_id, *kind)
            }
            Command::ApplyTransaction { transaction } => {
                Self::apply_transaction(state, envelope.sequence(), transaction)
            }
        }
    }

    fn next_revision(state: &WorldState) -> Result<WorldRevision, ApplyError> {
        state
            .revision()
            .get()
            .checked_add(1)
            .map(WorldRevision::new)
            .ok_or(ApplyError::RevisionOverflow)
    }

    fn advance_to(state: &mut WorldState, requested: SimTick) -> Result<(), ApplyError> {
        let current = state.tick();
        if requested < current {
            return Err(ApplyError::TimeRegression { current, requested });
        }

        let revision = Self::next_revision(state)?;
        state.commit_advance(requested, revision);
        Ok(())
    }

    fn schedule_event(
        state: &mut WorldState,
        event_id: &EventId,
        at_tick: SimTick,
        domain: EventDomain,
    ) -> Result<(), ApplyError> {
        let current = state.tick();
        if at_tick < current {
            return Err(ApplyError::EventInPast {
                current,
                requested: at_tick,
            });
        }

        let sequence = state
            .scheduler()
            .next_sequence()
            .ok_or(ApplyError::EventSequenceOverflow)?;
        let generation = state
            .scheduler()
            .next_generation(event_id)
            .ok_or_else(|| ApplyError::EventGenerationOverflow(event_id.clone()))?;
        let revision = Self::next_revision(state)?;

        state.commit_schedule_event(
            event_id.clone(),
            at_tick,
            domain,
            sequence,
            generation,
            revision,
        );
        Ok(())
    }

    fn cancel_event(state: &mut WorldState, event_id: &EventId) -> Result<(), ApplyError> {
        if !state.scheduler().is_active(event_id) {
            return Err(ApplyError::UnknownActiveEvent(event_id.clone()));
        }

        let generation = state
            .scheduler()
            .next_generation(event_id)
            .ok_or_else(|| ApplyError::EventGenerationOverflow(event_id.clone()))?;
        let revision = Self::next_revision(state)?;

        state.commit_cancel_event(event_id, generation, revision);
        Ok(())
    }

    fn open_inventory_account(
        state: &mut WorldState,
        account_id: &InventoryAccountId,
        kind: InventoryAccountKind,
    ) -> Result<(), ApplyError> {
        if state.inventory_account(account_id).is_some() {
            return Err(ApplyError::DuplicateInventoryAccount(account_id.clone()));
        }

        let revision = Self::next_revision(state)?;
        state.commit_inventory_account(InventoryAccount::new(account_id.clone(), kind), revision);
        Ok(())
    }

    fn open_money_account(
        state: &mut WorldState,
        account_id: &MoneyAccountId,
        kind: MoneyAccountKind,
    ) -> Result<(), ApplyError> {
        if state.money_account(account_id).is_some() {
            return Err(ApplyError::DuplicateMoneyAccount(account_id.clone()));
        }

        let revision = Self::next_revision(state)?;
        state.commit_money_account(MoneyAccount::new(account_id.clone(), kind), revision);
        Ok(())
    }

    fn apply_transaction(
        state: &mut WorldState,
        command_sequence: u64,
        transaction: &EconomicTransaction,
    ) -> Result<(), ApplyError> {
        Self::validate_transaction(state, transaction)?;
        let revision = Self::next_revision(state)?;
        state.commit_transaction(transaction, command_sequence, revision);
        Ok(())
    }

    fn validate_transaction(
        state: &WorldState,
        transaction: &EconomicTransaction,
    ) -> Result<(), ApplyError> {
        if transaction.is_empty() {
            return Err(ApplyError::EmptyTransaction);
        }

        if state.has_applied_transaction(transaction.id()) {
            return Err(ApplyError::DuplicateTransaction(transaction.id().clone()));
        }

        let mut commodity_net: BTreeMap<CommodityId, i64> = BTreeMap::new();
        let mut inventory_delta: BTreeMap<(InventoryAccountId, CommodityId), i64> = BTreeMap::new();

        for posting in transaction.inventory_postings() {
            if state.inventory_account(&posting.account_id).is_none() {
                return Err(ApplyError::UnknownInventoryAccount(
                    posting.account_id.clone(),
                ));
            }

            let net = commodity_net
                .entry(posting.commodity_id.clone())
                .or_insert(0);
            *net = net
                .checked_add(posting.delta.get())
                .ok_or(ApplyError::ArithmeticOverflow)?;

            let delta = inventory_delta
                .entry((posting.account_id.clone(), posting.commodity_id.clone()))
                .or_insert(0);
            *delta = delta
                .checked_add(posting.delta.get())
                .ok_or(ApplyError::ArithmeticOverflow)?;
        }

        for (commodity_id, net) in commodity_net {
            if net != 0 {
                return Err(ApplyError::InventoryNotConserved {
                    commodity_id,
                    net: Quantity::new(net),
                });
            }
        }

        for ((account_id, commodity_id), delta) in inventory_delta {
            let account = state
                .inventory_account(&account_id)
                .expect("inventory account was validated above");
            let attempted = account
                .balance(&commodity_id)
                .get()
                .checked_add(delta)
                .ok_or(ApplyError::ArithmeticOverflow)?;

            if account.kind() == InventoryAccountKind::Holding && attempted < 0 {
                return Err(ApplyError::NegativeInventoryBalance {
                    account_id,
                    commodity_id,
                    attempted: Quantity::new(attempted),
                });
            }
        }

        let mut money_net = 0_i128;
        let mut money_delta: BTreeMap<MoneyAccountId, i128> = BTreeMap::new();

        for posting in transaction.money_postings() {
            if state.money_account(&posting.account_id).is_none() {
                return Err(ApplyError::UnknownMoneyAccount(posting.account_id.clone()));
            }

            money_net = money_net
                .checked_add(posting.delta.get())
                .ok_or(ApplyError::ArithmeticOverflow)?;

            let delta = money_delta.entry(posting.account_id.clone()).or_insert(0);
            *delta = delta
                .checked_add(posting.delta.get())
                .ok_or(ApplyError::ArithmeticOverflow)?;
        }

        if money_net != 0 {
            return Err(ApplyError::MoneyNotConserved {
                net: MoneyCp::new(money_net),
            });
        }

        for (account_id, delta) in money_delta {
            let account = state
                .money_account(&account_id)
                .expect("money account was validated above");
            let attempted = account
                .balance()
                .get()
                .checked_add(delta)
                .ok_or(ApplyError::ArithmeticOverflow)?;

            if account.kind() == MoneyAccountKind::Holding && attempted < 0 {
                return Err(ApplyError::NegativeMoneyBalance {
                    account_id,
                    attempted: MoneyCp::new(attempted),
                });
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{ApplyError, WorldReducer};
    use crate::{Command, CommandEnvelope, EventDomain, EventId, SimTick, WorldState};

    #[test]
    fn successful_command_advances_time_and_revision_once() {
        let mut state = WorldState::new(7);
        let command =
            CommandEnvelope::new(1, Command::AdvanceTo { tick: SimTick::new(12) });

        WorldReducer::apply(&mut state, &command).unwrap();

        assert_eq!(state.tick(), SimTick::new(12));
        assert_eq!(state.revision().get(), 1);
    }

    #[test]
    fn time_cannot_move_backwards() {
        let mut state = WorldState::new(7);
        WorldReducer::apply(
            &mut state,
            &CommandEnvelope::new(1, Command::AdvanceTo { tick: SimTick::new(12) }),
        )
        .unwrap();

        let error = WorldReducer::apply(
            &mut state,
            &CommandEnvelope::new(2, Command::AdvanceTo { tick: SimTick::new(11) }),
        )
        .unwrap_err();

        assert_eq!(
            error,
            ApplyError::TimeRegression {
                current: SimTick::new(12),
                requested: SimTick::new(11),
            }
        );
        assert_eq!(state.tick(), SimTick::new(12));
        assert_eq!(state.revision().get(), 1);
    }

    #[test]
    fn event_cannot_be_scheduled_in_the_past() {
        let mut state = WorldState::new(7);
        WorldReducer::apply(
            &mut state,
            &CommandEnvelope::new(1, Command::AdvanceTo { tick: SimTick::new(12) }),
        )
        .unwrap();

        let error = WorldReducer::apply(
            &mut state,
            &CommandEnvelope::new(
                2,
                Command::ScheduleEvent {
                    event_id: EventId::new("late"),
                    at_tick: SimTick::new(11),
                    domain: EventDomain::System,
                },
            ),
        )
        .unwrap_err();

        assert_eq!(
            error,
            ApplyError::EventInPast {
                current: SimTick::new(12),
                requested: SimTick::new(11),
            }
        );
    }
}
