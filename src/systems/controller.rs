use crate::components::Controller;
use amethyst::core::Time;
use amethyst::core::{math::Vector3, Float, Transform};
use amethyst::ecs::{Join, Read, System, WriteStorage};

#[derive(Default, Debug)]
pub struct ControllerSystem;

impl<'a> System<'a> for ControllerSystem {
    type SystemData = (
        WriteStorage<'a, Controller>,
        WriteStorage<'a, Transform>,
        Read<'a, Time>,
    );

    fn run(&mut self, (mut controllers, mut transforms, time): Self::SystemData) {
        for (controller, transform) in (&mut controllers, &mut transforms).join() {
            // rotate based on unit points
            transform.append_rotation_y_axis(
                // This will orient the rotation direction correctly
                controller.rotation_control *
                // Multiply by our turn speed, which is just a multiplier.
                Float::from(controller.turn_speed) *
                // Finally, multiply everything by our delta to keep consistent across framerates
                Float::from(time.delta_seconds()),
            );

            // If our input is 0, we're not changing our velocity.
            if controller.thrust_control != Float::from(0.) {
                // Calculate impulse
                let added_magnitude = Vector3::z().scale(
                    controller.traction
                        * Float::from(time.delta_seconds())
                        * controller.thrust_control,
                );
                let added_vector = transform.rotation() * added_magnitude;

                // Change our velocity vector
                controller.velocity += added_vector;

                // Limit velocity
                let magnitude = controller.velocity.magnitude();
                if magnitude > controller.max_speed {
                    controller.velocity /= magnitude / controller.max_speed;
                }
            }

            // Apply existing velocity and rotational velocity.
            let movement = controller.velocity.scale(Float::from(time.delta_seconds()));

            // Finally, actually transform
            transform.prepend_translation(movement);
        }
    }
}
