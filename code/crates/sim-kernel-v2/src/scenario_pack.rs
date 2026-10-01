use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::{
    ApplyError, Command, CommandEnvelope, CommodityDef, CommodityId, EconomicTransaction,
    InventoryAccountId, InventoryAccountKind, InventoryPosting, MarketDef, MoneyAccountId,
    MoneyAccountKind, MoneyCp, MoneyPosting, PlaceDef, Quantity, RegistryError, RouteDef,
    ScenarioPackId, ScenarioRegistry, TransactionId, UnitDef, WorldReducer, WorldState,
};

pub const SCENARIO_PACK_SCHEMA_VERSION: u32 = 1;
pub const SEED_INVENTORY_SOURCE_ACCOUNT: &str = "__system.seed.inventory";
pub const SEED_MONEY_EXTERNAL_ACCOUNT: &str = "__system.seed.money";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ScenarioManifest {
    pub schema_version: u32,
    pub pack_id: ScenarioPackId,
    pub revision: u32,
    pub campaign_epoch: String,
    pub world_seed: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OpeningInventoryBalance {
    pub commodity_id: CommodityId,
    pub quantity: Quantity,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct InventoryAccountSeed {
    pub id: InventoryAccountId,
    pub kind: InventoryAccountKind,
    pub opening_balances: Vec<OpeningInventoryBalance>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MoneyAccountSeed {
    pub id: MoneyAccountId,
    pub kind: MoneyAccountKind,
    pub opening_balance: MoneyCp,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ScenarioPack {
    pub manifest: ScenarioManifest,
    pub units: Vec<UnitDef>,
    pub commodities: Vec<CommodityDef>,
    pub places: Vec<PlaceDef>,
    pub markets: Vec<MarketDef>,
    pub routes: Vec<RouteDef>,
    pub inventory_accounts: Vec<InventoryAccountSeed>,
    pub money_accounts: Vec<MoneyAccountSeed>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoadedScenario {
    manifest: ScenarioManifest,
    registry: ScenarioRegistry,
    world_state: WorldState,
}

impl LoadedScenario {
    #[must_use]
    pub fn manifest(&self) -> &ScenarioManifest {
        &self.manifest
    }

    #[must_use]
    pub fn registry(&self) -> &ScenarioRegistry {
        &self.registry
    }

    #[must_use]
    pub fn world_state(&self) -> &WorldState {
        &self.world_state
    }

    #[must_use]
    pub fn into_parts(self) -> (ScenarioManifest, ScenarioRegistry, WorldState) {
        (self.manifest, self.registry, self.world_state)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ScenarioPackError {
    Json(String),
    UnsupportedSchemaVersion {
        found: u32,
    },
    EmptyPackId,
    EmptyCampaignEpoch,
    Registry(RegistryError),
    DuplicateInventoryAccount(InventoryAccountId),
    DuplicateMoneyAccount(MoneyAccountId),
    ReservedInventoryAccount(InventoryAccountId),
    ReservedMoneyAccount(MoneyAccountId),
    DuplicateOpeningCommodity {
        account_id: InventoryAccountId,
        commodity_id: CommodityId,
    },
    UnknownOpeningCommodity {
        account_id: InventoryAccountId,
        commodity_id: CommodityId,
    },
    NegativeOpeningInventory {
        account_id: InventoryAccountId,
        commodity_id: CommodityId,
        quantity: Quantity,
    },
    NegativeOpeningMoney {
        account_id: MoneyAccountId,
        amount: MoneyCp,
    },
    SequenceOverflow,
    Reducer(ApplyError),
}

impl From<RegistryError> for ScenarioPackError {
    fn from(value: RegistryError) -> Self {
        Self::Registry(value)
    }
}

impl From<ApplyError> for ScenarioPackError {
    fn from(value: ApplyError) -> Self {
        Self::Reducer(value)
    }
}

impl ScenarioPack {
    pub fn from_json(json: &str) -> Result<Self, ScenarioPackError> {
        serde_json::from_str(json).map_err(|error| ScenarioPackError::Json(error.to_string()))
    }

    pub fn to_json_pretty(&self) -> Result<String, ScenarioPackError> {
        serde_json::to_string_pretty(self)
            .map_err(|error| ScenarioPackError::Json(error.to_string()))
    }

    pub fn validate(&self) -> Result<ScenarioRegistry, ScenarioPackError> {
        if self.manifest.schema_version != SCENARIO_PACK_SCHEMA_VERSION {
            return Err(ScenarioPackError::UnsupportedSchemaVersion {
                found: self.manifest.schema_version,
            });
        }

        if self.manifest.pack_id.as_str().trim().is_empty() {
            return Err(ScenarioPackError::EmptyPackId);
        }

        if self.manifest.campaign_epoch.trim().is_empty() {
            return Err(ScenarioPackError::EmptyCampaignEpoch);
        }

        let registry = ScenarioRegistry::new(
            self.units.clone(),
            self.commodities.clone(),
            self.places.clone(),
            self.markets.clone(),
            self.routes.clone(),
        )?;

        self.validate_inventory_accounts(&registry)?;
        self.validate_money_accounts()?;

        Ok(registry)
    }

    pub fn load(&self) -> Result<LoadedScenario, ScenarioPackError> {
        let registry = self.validate()?;
        let mut world_state = WorldState::new(self.manifest.world_seed);
        let mut sequence = 0_u64;

        Self::apply_command(
            &mut world_state,
            &mut sequence,
            Command::OpenInventoryAccount {
                account_id: InventoryAccountId::new(SEED_INVENTORY_SOURCE_ACCOUNT),
                kind: InventoryAccountKind::SourceOrSink,
            },
        )?;

        Self::apply_command(
            &mut world_state,
            &mut sequence,
            Command::OpenMoneyAccount {
                account_id: MoneyAccountId::new(SEED_MONEY_EXTERNAL_ACCOUNT),
                kind: MoneyAccountKind::External,
            },
        )?;

        let mut inventory_accounts = self.inventory_accounts.clone();
        inventory_accounts.sort_by(|left, right| left.id.cmp(&right.id));

        for account in &inventory_accounts {
            Self::apply_command(
                &mut world_state,
                &mut sequence,
                Command::OpenInventoryAccount {
                    account_id: account.id.clone(),
                    kind: account.kind,
                },
            )?;
        }

        let mut money_accounts = self.money_accounts.clone();
        money_accounts.sort_by(|left, right| left.id.cmp(&right.id));

        for account in &money_accounts {
            Self::apply_command(
                &mut world_state,
                &mut sequence,
                Command::OpenMoneyAccount {
                    account_id: account.id.clone(),
                    kind: account.kind,
                },
            )?;
        }

        for account in &inventory_accounts {
            let mut balances = account.opening_balances.clone();
            balances.sort_by(|left, right| left.commodity_id.cmp(&right.commodity_id));

            for balance in balances {
                if balance.quantity == Quantity::ZERO {
                    continue;
                }

                let transaction = EconomicTransaction::new(
                    TransactionId::new(format!(
                        "seed.inventory.{}.{}",
                        account.id, balance.commodity_id
                    )),
                    vec![
                        InventoryPosting::new(
                            InventoryAccountId::new(SEED_INVENTORY_SOURCE_ACCOUNT),
                            balance.commodity_id.clone(),
                            Quantity::new(
                                balance
                                    .quantity
                                    .get()
                                    .checked_neg()
                                    .ok_or(ScenarioPackError::SequenceOverflow)?,
                            ),
                        ),
                        InventoryPosting::new(
                            account.id.clone(),
                            balance.commodity_id,
                            balance.quantity,
                        ),
                    ],
                    vec![],
                );

                Self::apply_command(
                    &mut world_state,
                    &mut sequence,
                    Command::ApplyTransaction { transaction },
                )?;
            }
        }

        for account in &money_accounts {
            if account.opening_balance == MoneyCp::ZERO {
                continue;
            }

            let external_delta = account
                .opening_balance
                .get()
                .checked_neg()
                .ok_or(ScenarioPackError::SequenceOverflow)?;

            let transaction = EconomicTransaction::new(
                TransactionId::new(format!("seed.money.{}", account.id)),
                vec![],
                vec![
                    MoneyPosting::new(
                        MoneyAccountId::new(SEED_MONEY_EXTERNAL_ACCOUNT),
                        MoneyCp::new(external_delta),
                    ),
                    MoneyPosting::new(account.id.clone(), account.opening_balance),
                ],
            );

            Self::apply_command(
                &mut world_state,
                &mut sequence,
                Command::ApplyTransaction { transaction },
            )?;
        }

        Ok(LoadedScenario {
            manifest: self.manifest.clone(),
            registry,
            world_state,
        })
    }

    fn validate_inventory_accounts(
        &self,
        registry: &ScenarioRegistry,
    ) -> Result<(), ScenarioPackError> {
        let reserved = InventoryAccountId::new(SEED_INVENTORY_SOURCE_ACCOUNT);
        let mut seen_accounts = BTreeSet::new();

        for account in &self.inventory_accounts {
            if account.id == reserved {
                return Err(ScenarioPackError::ReservedInventoryAccount(
                    account.id.clone(),
                ));
            }

            if !seen_accounts.insert(account.id.clone()) {
                return Err(ScenarioPackError::DuplicateInventoryAccount(
                    account.id.clone(),
                ));
            }

            let mut seen_commodities = BTreeSet::new();
            for balance in &account.opening_balances {
                if !seen_commodities.insert(balance.commodity_id.clone()) {
                    return Err(ScenarioPackError::DuplicateOpeningCommodity {
                        account_id: account.id.clone(),
                        commodity_id: balance.commodity_id.clone(),
                    });
                }

                if !registry.commodities().contains_key(&balance.commodity_id) {
                    return Err(ScenarioPackError::UnknownOpeningCommodity {
                        account_id: account.id.clone(),
                        commodity_id: balance.commodity_id.clone(),
                    });
                }

                if account.kind == InventoryAccountKind::Holding && balance.quantity.get() < 0 {
                    return Err(ScenarioPackError::NegativeOpeningInventory {
                        account_id: account.id.clone(),
                        commodity_id: balance.commodity_id.clone(),
                        quantity: balance.quantity,
                    });
                }
            }
        }

        Ok(())
    }

    fn validate_money_accounts(&self) -> Result<(), ScenarioPackError> {
        let reserved = MoneyAccountId::new(SEED_MONEY_EXTERNAL_ACCOUNT);
        let mut seen_accounts = BTreeSet::new();

        for account in &self.money_accounts {
            if account.id == reserved {
                return Err(ScenarioPackError::ReservedMoneyAccount(account.id.clone()));
            }

            if !seen_accounts.insert(account.id.clone()) {
                return Err(ScenarioPackError::DuplicateMoneyAccount(account.id.clone()));
            }

            if account.kind == MoneyAccountKind::Holding && account.opening_balance.get() < 0 {
                return Err(ScenarioPackError::NegativeOpeningMoney {
                    account_id: account.id.clone(),
                    amount: account.opening_balance,
                });
            }
        }

        Ok(())
    }

    fn apply_command(
        state: &mut WorldState,
        sequence: &mut u64,
        command: Command,
    ) -> Result<(), ScenarioPackError> {
        *sequence = sequence
            .checked_add(1)
            .ok_or(ScenarioPackError::SequenceOverflow)?;

        WorldReducer::apply(state, &CommandEnvelope::new(*sequence, command))?;
        Ok(())
    }
}
