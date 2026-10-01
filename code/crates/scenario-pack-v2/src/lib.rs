mod compile;
mod constants;
mod error;
mod model;
mod registry;
mod validate;

pub use compile::compile_initialization_commands;
pub use error::{InitializeError, PackLoadError, ValidationError, ValidationIssue};
pub use model::{
    CommoditySpec, InventoryAccountSpec, MarketSpec, MoneyAccountSpec, OpeningInventorySpec,
    OpeningMoneySpec, PackManifest, PlaceSpec, ProvenanceStatus, RouteSpec, ScenarioPack,
    UnitSpec, SCENARIO_PACK_SCHEMA_VERSION,
};
pub use registry::compile_registry;
pub use validate::validate;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedScenarioPack {
    pack: ScenarioPack,
}

impl ValidatedScenarioPack {
    #[must_use]
    pub const fn pack(&self) -> &ScenarioPack {
        &self.pack
    }

    #[must_use]
    pub const fn world_seed(&self) -> u64 {
        self.pack.manifest.world_seed
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializedScenario {
    pack: ValidatedScenarioPack,
    registry: sim_kernel_v2::ScenarioRegistry,
    world_state: sim_kernel_v2::WorldState,
}

impl InitializedScenario {
    #[must_use]
    pub const fn pack(&self) -> &ValidatedScenarioPack {
        &self.pack
    }

    #[must_use]
    pub const fn registry(&self) -> &sim_kernel_v2::ScenarioRegistry {
        &self.registry
    }

    #[must_use]
    pub const fn world_state(&self) -> &sim_kernel_v2::WorldState {
        &self.world_state
    }

    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        ValidatedScenarioPack,
        sim_kernel_v2::ScenarioRegistry,
        sim_kernel_v2::WorldState,
    ) {
        (self.pack, self.registry, self.world_state)
    }
}

pub fn validate_pack(pack: ScenarioPack) -> Result<ValidatedScenarioPack, ValidationError> {
    validate(&pack)?;
    Ok(ValidatedScenarioPack { pack })
}

pub fn load_json(input: &str) -> Result<ValidatedScenarioPack, PackLoadError> {
    let pack: ScenarioPack =
        serde_json::from_str(input).map_err(|error| PackLoadError::Parse(error.to_string()))?;
    validate_pack(pack).map_err(PackLoadError::Validation)
}

pub fn initialize(pack: ValidatedScenarioPack) -> Result<InitializedScenario, InitializeError> {
    let registry = compile_registry(&pack)?;
    let commands = compile_initialization_commands(&pack);
    let world_state = sim_kernel_v2::replay(
        &sim_kernel_v2::WorldState::new(pack.world_seed()),
        &commands,
    )?;

    Ok(InitializedScenario {
        pack,
        registry,
        world_state,
    })
}
