use crate::assets::prefab::EntityPrefabs;
use amethyst::prelude::*;
use amethyst::{
    assets::Prefab,
    core::math::{Point3, Vector3},
    core::{Float, Transform},
    ecs::{Entities, Entity, Read, ReadExpect, WriteStorage},

};
use specs_physics::{
    bodies::{BodyStatus, Position},
    colliders::Shape,
    physics_dispatcher, PhysicsBodyBuilder, PhysicsColliderBuilder,
};


use crate::assets::prefab::EntityPrefabData;
use crate::components as c;
pub struct MainGameState {}

impl SimpleState for MainGameState {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;

        create_with_prefabs(world, vec!["game::camera"]);
        create_with_prefabs(world, vec!["game::light"]);

        let player = create_with_prefabs(
            world,
            vec![
                "game::player",
                "items::engines::default",
                "items::hulls::default",
            ],
        );
    }
}

pub fn create_with_prefabs(world: &mut World, paths: Vec<&str>) -> Entity {
    let mut prefab_handles = Vec::new();
    world.exec(|prefab_store: ReadExpect<EntityPrefabs>| {
        for path in paths {
            prefab_handles.push(
                prefab_store
                    .get_prefab(path)
                    .expect(&format!("Getting prefab with key {} failed.", path))
                    .clone(),
            )
        }
    });

    let mut entity_builder = world.create_entity();
    for prefab_handle in prefab_handles {
        entity_builder = entity_builder.with(prefab_handle);
    }
    entity_builder.build()
}