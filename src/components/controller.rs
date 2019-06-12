//! Controls and stores data for a movement
use amethyst::{
    assets::PrefabData,
    core::{
        math::{Unit, Vector3},
        Float,
    },
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    error::Error,
};
use serde::{Deserialize, Serialize};

/// Controllers must contain hulls to function properly
#[derive(Clone, Deserialize, Serialize, PrefabData)]
#[serde(default)]
#[prefab(Component)]
pub struct Controller {
    /// Preserves current velocity
    pub velocity: Unit<Vector3<Float>>,
    pub rotation_control: Float,
    pub thrust_control: Float,
    pub turn_speed: Float,
    pub traction: Float,
    pub max_speed: Float,
}

impl Default for Controller {
    fn default() -> Self {
        Self {
            velocity: Unit::new_unchecked(Vector3::zeros()),
            rotation_control: Float::from(0.0),
            thrust_control: Float::from(0.0),
            turn_speed: Float::from(1.0),
            traction: Float::from(0.05),
            max_speed: Float::from(10.0),
        }
    }
}

impl Component for Controller {
    type Storage = DenseVecStorage<Self>;
}

impl Controller {}
