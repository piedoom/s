use crate::assets::prefab::EntityPrefabs;
use amethyst::prelude::*;
use amethyst::{
    core::math::{Vector3, Point3},
    core::{Float, Transform},
    renderer::camera::{Camera, Projection},
};
use specs_physics::{
    bodies::{BodyStatus, Position},
    colliders::Shape,
    physics_dispatcher,
    PhysicsBodyBuilder,
    PhysicsColliderBuilder,
};

use crate::components as c;

pub struct MainGameState {}

impl SimpleState for MainGameState {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;

        // initialize nphysics
        // create the dispatcher containing all relevant Systems; alternatively to using
        // the convenience function you can add all required Systems by hand
        let mut dispatcher = physics_dispatcher::<f32, c::Position>();
        dispatcher.setup(&mut world.res);

        let camera = world
            .create_entity()
            .with(Camera::from(Projection::perspective(
                1.3,
                std::f32::consts::FRAC_PI_3,
                0.1,
                1000.0,
            )))
            .with(Transform::from(Vector3::new(
                Float::from(0.0),
                Float::from(0.0),
                Float::from(1.0),
            )))
            .build();

        let player = {
            let prefabs = world.read_resource::<EntityPrefabs>();
            prefabs.get_prefab("player").unwrap().clone()
        };
        let light = {
            let prefabs = world.read_resource::<EntityPrefabs>();
            prefabs.get_prefab("point_light").unwrap().clone()
        };

        let physics_body = PhysicsBodyBuilder::from(BodyStatus::Dynamic)
            .velocity(Vector3::new(1.0f32, 1.0f32, 1.0f32))
            .build();

        world.create_entity()
            .with(player.clone())
            .with(physics_body)
            .build();
        world.create_entity().with(light.clone()).build();
        dispatcher.dispatch(&world.res);
    }
}
