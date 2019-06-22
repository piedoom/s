use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter, RonFormat},
    core::ecs::{Write, WriteExpect},
    prelude::*,
    renderer::{
        formats::texture::ImageFormat,
        sprite::{SpriteSheet, SpriteSheetFormat},
        Texture,
    },
};

use crate::assets::prefab::{initialize_prefabs, update_prefabs};
use crate::resources::SResource;
use crate::states::main::MainGameState;
use amethyst::utils::application_root_dir;
use std::fs::read_dir;
use walkdir::WalkDir;

const TEXTURES_PATH: &'static str = "resources/textures";
const PREFABS_PATH: &'static str = "resources/prefabs";

/// This initial loadstate will load a bunch of paths that we will use to load further assets.
#[derive(Default)]
pub struct LoadInitialState {
    pub progress: ProgressCounter,
}

impl SimpleState for LoadInitialState {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        let world = &mut data.world;

        // -------------------------- //
        // --- Sprite Sheet Loads --- //
        // -------------------------- //
        type SSData<'s> = (
            WriteExpect<'s, Loader>,
            Write<'s, AssetStorage<Texture>>,
            Write<'s, AssetStorage<SpriteSheet>>,
        );

        let sheet_resource = world.exec(|(mut loader, mut textures, mut sprite_sheets): SSData| {
            let mut sheet_resource = SResource::default();
            // get folders, and only ron files
            let sprite_sheets_dir_path = application_root_dir().unwrap().join(TEXTURES_PATH);
            let sprite_sheets_iter = WalkDir::new(sprite_sheets_dir_path).into_iter().filter_map(|e| {
                // discard if error or directory
                if e.is_ok() {
                    let r = e.unwrap();
                    if r.file_type().is_file() {
                        return Some(r);
                    }
                }
                None
            });

            for entry in sprite_sheets_iter {
                let extension = entry
                    .path()
                    .extension()
                    .expect("Could not read file extension.")
                    .to_string_lossy()
                    .to_string();
                // break early if not a ron file
                if &extension != "ron" {
                    break;
                }
                // The key will always be the full path minus any extension
                let k = entry
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                // Our value here will always be the texture handle
                let v = {
                    let texture_handle = loader.load(
                        entry.path().join(".png").to_string_lossy(),
                        ImageFormat::default(),
                        &mut self.progress,
                        &textures,
                    );
                    loader.load(
                        entry.path().join(".ron").to_string_lossy(),
                        SpriteSheetFormat(texture_handle),
                        &mut self.progress,
                        &sprite_sheets,
                    )
                };
                // Insert these new values into our resource
                sheet_resource.insert(k.clone(), v);
            }
            sheet_resource
        });
        data.world.add_resource(sheet_resource);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress.is_complete() {
            Trans::Switch(Box::new(LoadState::default()))
        } else {
            Trans::None
        }
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
