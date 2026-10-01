mod compile;
mod constants;
mod error;
mod model;
mod registry;
mod validate;

pub use compile::compile_initialization_commands;
pub use error::{PackLoadError, ValidationError, ValidationIssue};
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

pub fn validate_pack(pack: ScenarioPack) -> Result<ValidatedScenarioPack, ValidationError> {
    validate(&pack)?;
    Ok(ValidatedScenarioPack { pack })
}

pub fn load_json(input: &str) -> Result<ValidatedScenarioPack, PackLoadError> {
    let pack: ScenarioPack =
        serde_json::from_str(input).map_err(|error| PackLoadError::Parse(error.to_string()))?;
    validate_pack(pack).map_err(PackLoadError::Validation)
}
