//! Simply marks something as a controller that can recieve input
use amethyst::ecs::{Component, DenseVecStorage};

pub struct Player {
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
