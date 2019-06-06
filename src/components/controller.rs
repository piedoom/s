//! Controls and stores data for a movement
use amethyst::{
    core::{
        math::{Unit, Vector3},
        Float,
    },
    ecs::{Component, DenseVecStorage},
};

#[derive(Debug)]
pub struct Controller {
    pub direction: Unit<Vector3<Float>>,
}

impl Component for Controller {
    type Storage = DenseVecStorage<Self>;
}

impl Controller {
    pub fn set_direction(&mut self, direction: Unit<Vector3<Float>>) {
        self.direction = direction;
    }
}