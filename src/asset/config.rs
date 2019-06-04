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
    pub prefabs: PrefabPathsConfig,
}

/// paths to all Sprite Sheets
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PrefabPathsConfig {
    pub player: PathBuf,
}

impl<'a> Default for PrefabPathsConfig {
    fn default() -> Self {
        Self {
            player: PathBuf::new()
        }
    }
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
