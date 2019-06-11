use amethyst::{
    assets::PrefabData,
    error::Error,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Default)]
pub struct Inventory{ }
