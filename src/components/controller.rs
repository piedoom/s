//! Controls and stores data for a movement
use crate::components::weapon::Projectile;
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
    pub velocity: Vector3<Float>,
    pub rotation_control: Float,
    pub thrust_control: Float,
    pub turn_speed: Float,
    pub traction: Float,
    pub max_speed: Float,
}

impl Default for Controller {
    fn default() -> Self {
        Self {
            velocity: Vector3::zeros(),
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

impl Controller {
    /// When we fire, we want to impart data from our projectile onto our controller
    pub fn set_from_projectile(mut self, projectile: &Projectile) -> Self {
        self.max_speed = projectile.max_speed;
        self.traction = projectile.traction;
        // Projectile should travel along continuously
        self.thrust_control = Float::from(-1.0);
        self.rotation_control = Float::from(0.0);
        self
    }
}
