use std::path::PathBuf;
use amethyst::{
    assets::{
        Handle,
        Asset,
        ProcessingState,
    },
    error::Error,
    ecs::VecStorage,
};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct GameConfig {
    /// Path to all prefabs we want to load
    pub prefabs: Vec<String>,
}

impl Asset for GameConfig {
    const NAME: &'static str = "s::GameConfig";
    // use `Self` if the type is directly serialized.
    type Data = GameConfig;
    type HandleStorage = VecStorage<GameConfigHandle>;
}

impl From<GameConfig> for Result<ProcessingState<GameConfig>, Error> {
    fn from(config: GameConfig) -> Result<ProcessingState<GameConfig>, Error> {
        Ok(ProcessingState::Loaded(config))
    }
}

/// A handle to a `PrefabPathsConfig` asset.
pub type GameConfigHandle = Handle<GameConfig>;
