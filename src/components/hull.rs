use amethyst::{
    assets::PrefabData,
    core::Float,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    error::Error,
};
use serde::{Deserialize, Serialize};

/// Specifies core traits about a ship
#[derive(Clone, Deserialize, Serialize, PrefabData)]
#[serde(default)]
#[prefab(Component)]
pub struct Hull {
    // Will be used later when we have engines
    pub weight: usize,
    /// Used for calculating inventory space
    pub inventory_space: usize,
    pub max_speed: Float,
}

impl Default for Hull {
    fn default() -> Self {
        Self {
            weight: 100,
            inventory_space: 100,
            max_speed: Float::from(6.0),
        }
    }
}

impl Component for Hull {
    type Storage = DenseVecStorage<Self>;
}
