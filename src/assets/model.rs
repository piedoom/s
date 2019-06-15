use crate::assets::config::GameConfig;
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
        RonFormat, Loader,
    },
    controls::ControlTagPrefab,
    core::{
        ecs::Component,
        ecs::DenseVecStorage,
        ecs::Entity,
        ecs::{Read, WriteStorage, ReadExpect},
        Named, Transform,
    },
    derive::PrefabData,
    gltf::{GltfSceneAsset, GltfSceneFormat},
    prelude::*,
    utils::{tag::Tag, application_root_dir},
    Error,
};
use std::fs::read_dir;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
type GenMeshVertex = (Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>);
use crate::components as c;

pub fn initialize_models(world: &mut World, config_handle: Handle<GameConfig>) -> ProgressCounter {
    type Data<'a> = (
        Read<'a, AssetStorage<GameConfig>>,
        PrefabLoader<'a, GltfSceneAsset>,
        ReadExpect<'a, Loader>
    );

    let progress = world.exec(|(config_storage, model_storage, loader): Data| {
        let config = &config_storage.get(&config_handle.clone()).unwrap();
        let mut progress_counter = ProgressCounter::new();

        // Loop over all directories, assign name based on relative path, add to hashmap
        let models_path = application_root_dir().unwrap().join(config.models_path);
        let models_iter = read_dir(models_path).expect("Could not read models path.");

        let models_resource: HashMap<String, Handle<GltfSceneAsset>> = HashMap::new();
        // loop over and load all of our models
        models_iter.map(|model_path|{
            let path_str = get_key_name(model_path.unwrap());
            let handle = loader.load(
                path_str,
                GltfSceneFormat,
                model_storage,
                &mut progress_counter
            );
            models_resource.insert(path_str, handle);
        });


        let models_iter = {
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

/// Convert a path into a string name.
fn get_key_name(entry: std::fs::DirEntry) -> String {
    entry.path().into_os_string().into_string()
}