use amethyst::ecs::prelude::*;
use specs_physics::bodies as b;

#[derive(Default)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32,
}

impl Component for Position {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl b::Position<f32> for Position {
    fn position(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }

    fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }
}
