use amethyst::renderer::{
    camera::CameraPrefab,
    light::LightPrefab,
    sprite::prefab::{SpriteRenderPrefab, SpriteSheetPrefab},
    transparent::Transparent,
};
use amethyst::{
    assets::{AssetStorage, Handle, Prefab, PrefabData, PrefabLoader, ProgressCounter, RonFormat},
    core::{ecs::Entity, ecs::Read, Named, Transform},
    derive::PrefabData,
    prelude::*,
    Error,
};

use crate::components as c;
use amethyst::utils::application_root_dir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::read_dir;
use walkdir::WalkDir;

// This is the main prefab data for everything.
// Only define the ones you want to add to your entity.
#[derive(Deserialize, Serialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub struct EntityPrefabData {
    pub name: Option<Named>,
    sprite_sheet: Option<SpriteSheetPrefab>,
    sprite: Option<SpriteRenderPrefab>,
    camera: Option<CameraPrefab>,
    transform: Option<Transform>,
    light: Option<LightPrefab>,
    player: Option<c::Player>,
    controller: Option<c::Controller>,
    weapon_manager: Option<c::weapon::WeaponManager>,
}

/// Contains a map of every possible entity we can spawn
#[derive(Default)]
pub struct EntityPrefabs {
    prefabs: HashMap<String, Handle<Prefab<EntityPrefabData>>>,
}

impl EntityPrefabs {
    pub fn insert(&mut self, entity_type: String, prefab_handle: Handle<Prefab<EntityPrefabData>>) {
        self.prefabs.insert(entity_type, prefab_handle);
    }

    pub fn get_prefab(&self, entity_type: &str) -> Option<&Handle<Prefab<EntityPrefabData>>> {
        self.prefabs.get(entity_type)
    }

    pub fn get_prefabs(&self) -> &HashMap<String, Handle<Prefab<EntityPrefabData>>> {
        &self.prefabs
    }

    pub fn set_prefabs(&mut self, prefabs: HashMap<String, Handle<Prefab<EntityPrefabData>>>) {
        self.prefabs = prefabs;
    }
}

// Here we load all prefabs
// These prefabs are then stored in a resource of type EntityPrefabs that is used by the spawner system.
// At initialization time, we put temporary keys for the prefabs since they're not loaded yet.
// When their loading is finished, we read the name of the entity inside to change the keys. This is done in the update_prefabs function.
pub fn initialize_prefabs(world: &mut World, prefabs_path: &'static str) -> ProgressCounter {
    let (progress, prefabs_resource) = world.exec(|loader: PrefabLoader<EntityPrefabData>| {
        let mut prefabs_resource = EntityPrefabs::default();
        let mut progress_counter = ProgressCounter::new();

        // get the directory of prefabs
        let absolute_prefabs_path = application_root_dir().unwrap().join(prefabs_path);
        let prefabs = WalkDir::new(absolute_prefabs_path)
            .into_iter()
            .filter_map(|e| {
                // discard if error or directory
                if e.is_ok() {
                    let r = e.unwrap();
                    if r.file_type().is_file() {
                        return Some(r);
                    }
                }
                None
            })
            .map(|entry| {
                println!("{}", entry.path().display());
                loader.load(
                    entry.path().to_string_lossy(),
                    RonFormat,
                    &mut progress_counter,
                )
            });

        // Add the collection to a resource
        for (count, prefab) in prefabs.enumerate() {
            prefabs_resource.insert(format!("temp_prefab_{}", count), prefab);
        }

        (progress_counter, prefabs_resource)
    });

    world.add_resource(prefabs_resource);

    progress
}

// Once the prefabs are loaded, this function is called to update the ekeys in the CreaturePrefabs struct.
// We use the Named component of the entity to determine which key to use.
pub fn update_prefabs(world: &mut World) {
    let updated_prefabs = {
        let creature_prefabs = world.read_resource::<EntityPrefabs>();
        let prefabs = creature_prefabs.get_prefabs();
        let mut prefab_resource = world.write_resource::<AssetStorage<Prefab<EntityPrefabData>>>();
        let mut new_prefabs = HashMap::new();
        for (_, handle) in prefabs.iter() {
            if let Some(prefab) = prefab_resource.get_mut(handle) {
                if let Some(prefab_data) = prefab.entity(0) {
                    let name = prefab_data
                        .data()
                        .unwrap()
                        .name
                        .as_ref()
                        .unwrap()
                        .name
                        .to_string();
                    new_prefabs.insert(name, handle.clone());
                }
            }
        }
        new_prefabs
    };
    world
        .write_resource::<EntityPrefabs>()
        .set_prefabs(updated_prefabs);
}
