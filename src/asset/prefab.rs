use crate::asset::config::GameConfig;
use amethyst::renderer::{
    camera::CameraPrefab,
    formats::{mesh::MeshPrefab, mtl::MaterialPrefab},
    light::LightPrefab,
    rendy::mesh::{Normal, Position, Tangent, TexCoord},
    sprite::{
        prefab::{SpriteRenderPrefab, SpriteSheetPrefab},
        SpriteRender,
    },
    transparent::Transparent,
};
use amethyst::{
    animation::AnimationSetPrefab,
    assets::{
        AssetPrefab, AssetStorage, Handle, Prefab, PrefabData, PrefabLoader, ProgressCounter,
        RonFormat,
    },
    controls::ControlTagPrefab,
    core::{
        ecs::Component,
        ecs::DenseVecStorage,
        ecs::Entity,
        ecs::{Read, WriteStorage},
        Named, Transform,
    },
    gltf::{GltfSceneAsset, GltfSceneFormat},
    prelude::*,
    utils::tag::Tag,
    Error,
};

use amethyst::derive::PrefabData;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
type GenMeshVertex = (Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>);

// This is the main prefab data for everything.
// Only define the ones you want to add to your entity.
#[derive(Deserialize, Serialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub struct EntityPrefabData {
    pub name: Option<Named>,
    mesh: Option<MeshPrefab<GenMeshVertex>>,
    material: Option<MaterialPrefab>,
    gltf: Option<AssetPrefab<GltfSceneAsset, GltfSceneFormat>>,
    camera: Option<CameraPrefab>,
    transform: Option<Transform>,
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
pub fn initialize_prefabs(world: &mut World, config_handle: Handle<GameConfig>) -> ProgressCounter {
    type Data<'a> = (
        Read<'a, AssetStorage<GameConfig>>,
        PrefabLoader<'a, EntityPrefabData>,
    );
    let (progress, prefabs) = world.exec(|(config_storage, loader): Data| {
        let config = &config_storage.get(&config_handle.clone()).unwrap();
        let mut prefabs = EntityPrefabs::default();
        let mut progress_counter = ProgressCounter::new();

        // loop over and load all of our prefabs
        let prefab_iter = {
            config.prefabs.iter().map(|prefab_path| {
                loader.load(prefab_path.clone(), RonFormat, &mut progress_counter)
            })
        };

        // Add the collection to a resource
        for (count, prefab) in prefab_iter.enumerate() {
            prefabs.insert(format!("temp_prefab_{}", count), prefab);
        }

        (progress_counter, prefabs)
    });

    world.add_resource(prefabs);

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
