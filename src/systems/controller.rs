use crate::{
    components::{Controller, Player},
    data::Axis,
};
use amethyst::core::Time;
use amethyst::core::{
    math::{Unit, Vector3},
    Float, Transform,
};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

const UP: usize = 1;

use crate::data::GameBindings;

#[derive(Default, Debug)]
pub struct ControllerSystem;

impl<'a> System<'a> for ControllerSystem {
    type SystemData = (
        WriteStorage<'a, Controller>,
        WriteStorage<'a, Transform>,
        Read<'a, Time>,
        Read<'a, InputHandler<GameBindings>>,
    );

    fn run(&mut self, (mut controllers, mut transforms, time, input): Self::SystemData) {
        for (controller, transform) in (&mut controllers, &mut transforms).join() {
            // rotate based on unit points
            transform.append_rotation_z_axis(
                // This will orient the rotation direction correctly
                controller.rotation_control *
                // Multiply by our turn speed, which is just a multiplier.
                Float::from(controller.turn_speed) *
                // Finally, multiply everything by our delta to keep consistent across framerates
                Float::from(time.delta_seconds()),
            );

            let rotation = transform.isometry().inverse().rotation.to_homogeneous();
            let direction = Unit::new_unchecked(Vector3::new(rotation.row(UP)[0], rotation.row(UP)[1], Float::from(0.)));

            // If our input is 0, we're not changing our velocity.
            if controller.thrust_control != Float::from(0.) {
                let mut new_velocity = controller.velocity.as_ref() + direction.scale(controller.thrust_control * controller.acceleration);
                // cap the vector at 1
                if new_velocity.x > Float::from(1.) {
                    new_velocity.x = Float::from(1.);
                }
                if new_velocity.x < Float::from(-1.) {
                    new_velocity.x = Float::from(-1.);
                }
                if new_velocity.y > Float::from(1.) {
                    new_velocity.y = Float::from(1.);
                }
                if new_velocity.y < Float::from(-1.) {
                    new_velocity.y = Float::from(-1.);
                }

                // We know the values are capped, so no need to check.
                controller.velocity = Unit::new_unchecked(new_velocity);
            }


            // Finally, actually transform, multiplying by our max speed and delta
            transform.prepend_translation(
                controller
                    .velocity
                    .scale(controller.max_speed * Float::from(time.delta_seconds())),
            );
        }
    }
}
