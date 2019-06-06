use crate::{
    components::{Controller, Player},
    data::Axis,
};
use amethyst::core::{
    Transform,
    Float,
    math::{
        Vector3,
        Unit,
    }
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
            controller.set_direction(
                Unit::new_unchecked(
                    Vector3::new(
                        Float::from(input.axis_value(&Axis::Horizontal).unwrap()),
                        Float::from(input.axis_value(&Axis::Vertical).unwrap()),
                        Float::from(0.)
                    )
                )
            );
            dbg!(controller);
        }

        
    }
}
