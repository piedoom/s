use amethyst::core::{
    Time,
    Transform,
    Float,
    math::{
        Vector3,
        Unit,
    }
};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use crate::components as c;

#[derive(Default, Debug)]
pub struct WeaponSystem;

impl<'a> System<'a> for WeaponSystem {
    type SystemData = (
        WriteStorage<'a, c::weapon::WeaponManager>,
        ReadStorage<'a, Transform>,
        Read<'a, Time>
    );

    fn run(&mut self, (mut managers, transforms, time): Self::SystemData) {
        // Loop through all players and assign direction to their controller
        for (manager, transform) in (&mut managers, &transforms).join() {
            // We only care if the weapons manager wants to fire. Otherwise, do nothing.
            if manager.wants_to_fire {
                let current_time = time.absolute_time();

                // check if we can actually fire our weapon
                let poss_weapon = manager.current_weapon_mut();
                if let Some(weapon) = poss_weapon {
                    if current_time >= weapon.last_fired + weapon.recoil {
                        dbg!("Firing weapon!");
                        weapon.last_fired = current_time;
                    }
                }
            }
        }
    }
}
