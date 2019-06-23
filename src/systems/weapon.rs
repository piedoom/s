use crate::components as c;
use amethyst::core::{
    math::{Unit, Vector3},
    Float, Time, Transform,
};
use amethyst::{
    ecs::{prelude::*, world::LazyUpdate, Entities, Join, Read, System, WriteStorage},
    renderer::{SpriteRender},
};
use crate::assets::ResourceCollection;

#[derive(Default, Debug)]
pub struct WeaponSystem;

impl<'a> System<'a> for WeaponSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        WriteStorage<'a, c::weapon::WeaponManager>,
        WriteStorage<'a, c::Controller>,
        WriteStorage<'a, Transform>,
        Read<'a, Time>,
        Read<'a, ResourceCollection>,
    );

    fn run(
        &mut self,
        (entities, lazy, mut managers, mut controllers, mut transforms, time, res): Self::SystemData,
    ) {
        let controllers_looper = &mut controllers;
        // Loop through all players and assign direction to their controller
        for (manager, controller, transform) in
            (&mut managers, controllers_looper, &mut transforms).join()
        {
            // We only care if the weapons manager wants to fire. Otherwise, do nothing.
            if manager.wants_to_fire {
                let current_time = time.absolute_time();

                // check if we can actually fire our weapon
                let poss_weapon = manager.current_weapon_mut();
                if let Some(weapon) = poss_weapon {
                    if current_time >= weapon.last_fired + weapon.recoil {
                        // reset our recoil timer
                        weapon.last_fired = current_time;
                        // create an entity and assign it a clone of our weapon's `Projectile` component
                        let projectile = &weapon.projectile;

                        // What sprite to apply to the projectile entity
                        let sprite_render = SpriteRender {
                            sprite_sheet: res.projectile_sheet.clone().unwrap(),
                            sprite_number: 0, // First sprite
                        };

                        lazy.create_entity(&entities)
                            .with(projectile.clone())
                            .with(transform.clone())
                            .with(controller.clone().set_from_projectile(&projectile))
                            .with(sprite_render)
                            .build();
                    }
                }
            }
        }
    }
}
