use crate::{
    components::{Controller, Player},
    data::Axis,
};
use amethyst::core::{
    math::{Point2, Unit},
    Float, Transform,
};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

// You'll have to mark PADDLE_HEIGHT as public in pong.rs
use crate::data::GameBindings;

#[derive(Default, Debug)]
pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, Controller>,
        Read<'a, InputHandler<GameBindings>>,
    );

    fn run(&mut self, (players, mut controllers, input): Self::SystemData) {
        // Loop through all players and assign direction
        for (controller, _) in (&mut controllers, &players).join() {
            controller.rotation_control = Float::from(input.axis_value(&Axis::Horizontal).unwrap());
            controller.thrust_control = Float::from(input.axis_value(&Axis::Vertical).unwrap());
        }
    }
}
