use crate::asset::prefab::EntityPrefabs;
use amethyst::prelude::*;
use amethyst::{
    core::math::Vector3,
    core::{Float, Transform},
    renderer::camera::{Camera, Projection},
};

pub struct MainGameState {}

impl SimpleState for MainGameState {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;
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

        world.create_entity().with(player.clone()).build();
    }
}
