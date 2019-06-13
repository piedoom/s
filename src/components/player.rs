use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    error::Error,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, PrefabData, Default)]
#[serde(default)]
#[prefab(Component)]
pub struct Player { }

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
