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
pub struct Weapon {}

impl Component for Weapon {
    type Storage = DenseVecStorage<Self>;
}

/// A weapon manager references weapons that are currently in the inventory. It switches active weapon and also
/// equips/unequips weapons.
#[derive(Clone, Deserialize, Serialize, PrefabData, Default)]
#[serde(default)]
#[prefab(Component)]
pub struct WeaponManager {
    /// Currently Equipped Weapons
    weapons: Vec<&Weapon>,
    /// The active weapon in our weapons vec
    active: usize,
}