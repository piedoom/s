use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter, RonFormat},
    prelude::*,
    renderer::{
        formats::texture::ImageFormat,
        sprite::{SpriteSheet, SpriteSheetFormat},
        Texture,
    },
    ecs::{ReadExpect, Read},
};

use crate::assets::prefab::{initialize_prefabs, update_prefabs};
use crate::resources::SResource;
use crate::states::main::MainGameState;
use amethyst::utils::application_root_dir;
use std::fs::read_dir;
use walkdir::WalkDir;
use crate::assets::ResourceCollection;

const PREFABS_PATH: &'static str = "resources/prefabs";

/// This initial loadstate will load a bunch of paths that we will use to load further assets.
pub struct LoadInitialState {
    pub progress: ProgressCounter,
}

impl Default for LoadInitialState {
    fn default() -> Self {
        Self {
            progress: ProgressCounter::new(),
        }
    }
}

impl SimpleState for LoadInitialState {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        let world = &mut data.world;
        let mut progress = ProgressCounter::new();
        let mut resource_collection = ResourceCollection::default();
        self.progress = resource_collection.build(world, progress);
        resource_collection.register(world);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress.is_complete() {
            return Trans::Switch(Box::new(LoadState::default()))
        }
        return Trans::None
    }
}

#[derive(Default)]
pub struct LoadState {
    pub prefab_progress: Option<ProgressCounter>,
}

impl SimpleState for LoadState {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        // initialize the prefab resource
        self.prefab_progress = Some(initialize_prefabs(&mut data.world, PREFABS_PATH));
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // check to see if our prefabs are done loading
        if let Some(ref counter) = self.prefab_progress.as_ref() {
            if counter.is_complete() {
                // If so, reset our progress, and updae prefabs
                self.prefab_progress = None;
                update_prefabs(&mut data.world);
                // Create a new main state now that our resource is full of prefabs
                return Trans::Switch(Box::new(MainGameState {}));
            }
        }
        Trans::None
    }
}
