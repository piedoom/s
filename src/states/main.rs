use crate::assets::prefab::EntityPrefabData;
use crate::assets::prefab::EntityPrefabs;
use crate::components as c;
use amethyst::prelude::*;
use amethyst::{
    assets::{Handle, Prefab},
    core::math::{Point3, Vector3},
    core::{Float, Transform},
    ecs::{Entities, Entity, Read, ReadExpect, WriteStorage},
};
use specs_physics::{
    bodies::{BodyStatus, Position},
    colliders::Shape,
    physics_dispatcher, PhysicsBodyBuilder, PhysicsColliderBuilder,
};
pub struct MainGameState {}

impl SimpleState for MainGameState {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;
        world.register::<c::weapon::Projectile>();

        create_with_prefab(world, "game::camera");
        create_with_prefab(world, "game::light");
        create_with_prefab(world, "game::player");
    }
}

pub fn get_prefab(world: &mut World, path: &str) -> Handle<Prefab<EntityPrefabData>> {
    world.exec(|prefab_store: ReadExpect<EntityPrefabs>| {
        prefab_store
            .get_prefab(path)
            .expect(&format!("Getting prefab with key {} failed.", path))
            .clone()
    })
}

pub fn create_with_prefab(world: &mut World, path: &str) -> Entity {
    let prefab_handle = get_prefab(world, path).clone();
    world.create_entity().with(prefab_handle).build()
}
