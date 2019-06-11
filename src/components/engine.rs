use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    error::Error,
    core::Float,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, PrefabData)]
#[serde(default)]
#[prefab(Component)]
pub struct Engine {
    pub traction: Float,
    pub turn_speed: Float,
    pub speed_multiplier: Float,
}

impl Default for Engine {
    fn default() -> Self {
        Self {
            traction: Float::from(0.0),
            turn_speed: Float::from(0.0),
            speed_multiplier: Float::from(0.0),
        }
    }
}

impl Component for Engine {
    type Storage = DenseVecStorage<Self>;
}
