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
        world.add_resource({
            let loader = world.write_resource::<Loader>();
            let textures = world.write_resource::<AssetStorage<Texture>>();
            let sprite_sheets = world.write_resource::<AssetStorage<SpriteSheet>>();

            let mut sheet_resource = SResource::default();
            // get folders, and only ron files
            let sprite_sheets_dir_path = application_root_dir().unwrap().join(TEXTURES_PATH);
            for entry in WalkDir::new(sprite_sheets_dir_path.clone()).into_iter() {
                dbg!(&entry);
                let entry = entry.unwrap();
                if entry.clone().file_type().is_file() {
                    let extension = entry
                        .path()
                        .extension()
                        .expect("Could not read file extension.")
                        .to_string_lossy()
                        .to_string();
                    // break early if not a ron file
                    if &extension != "ron" {
                        continue;
                    }
                    let parent_dir = entry.path().parent().unwrap().to_string_lossy().to_string();
                    let file_stem = entry.path().file_stem().unwrap().to_string_lossy().to_string();
                    // The key will always be the full path minus any extension
                    let k = parent_dir + "/" + &file_stem;
                        
                    // Our value here will always be the texture handle
                    let v = {
                        let texture_handle = loader.load(
                            k.clone() + ".png",
                            ImageFormat::default(),
                            &mut self.progress,
                            &textures,
                        );
                        loader.load(
                            k.clone() + ".ron",
                            SpriteSheetFormat(texture_handle),
                            &mut self.progress,
                            &sprite_sheets,
                        )
                    };
                    // Insert these new values into our resource
                    sheet_resource.insert(k.clone(), v);
                }
            }
        });
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
