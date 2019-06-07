use amethyst::{
    assets::PrefabData,
    error::Error,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, PrefabData, Default)]
#[serde(default)]
#[prefab(Component)]
pub struct Player {}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
