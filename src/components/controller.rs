//! Controls and stores data for a movement
use amethyst::{
    core::{
        math::{Vector3, Unit},
        Float,
    },
    assets::PrefabData,
    error::Error,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, PrefabData)]
#[serde(default)]
#[prefab(Component)]
pub struct Controller {
    pub rotation_control: Float,
    pub thrust_control: Float,
    pub velocity: Unit<Vector3<Float>>,
    pub turn_speed: Float,
    /// Actually the speed to which our velocity will be scaled
    pub max_speed: Float,
    /// Increments our velocity
    pub acceleration: Float,
}

impl Default for Controller {
    fn default() -> Self {
        Self {
            rotation_control: Float::from(0.),
            thrust_control: Float::from(0.),
            velocity: Unit::new_unchecked(Vector3::zeros()),
            turn_speed: Float::from(1.),
            max_speed: Float::from(1.),
            acceleration: Float::from(0.05),
        }
    }
}

impl Component for Controller {
    type Storage = DenseVecStorage<Self>;
}

impl Controller {

}