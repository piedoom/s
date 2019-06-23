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

const PROJECTILES_PATH: &'static str = "textures/projectiles";
const PREFABS_PATH: &'static str = "resources/prefabs";

#[derive(Default)]
pub struct ResourceCollection {
    pub projectile_sheet: Option<Handle<SpriteSheet>>,
}

impl ResourceCollection {
    pub fn build(&mut self, world: &mut World, progress: ProgressCounter) -> ProgressCounter {
        let progress = self.register_projectile_sheets(world, progress);
        progress
    }

    pub fn register(self, world: &mut World) {
        world.add_resource(self);
    }

    fn register_projectile_sheets(&mut self, world: &mut World, mut progress: ProgressCounter) -> ProgressCounter {
        type SData<'s> = (
            ReadExpect<'s, Loader>,
            Read<'s, AssetStorage<Texture>>,
            Read<'s, AssetStorage<SpriteSheet>>,
        );
        self.projectile_sheet = Some(world.exec(|(loader, textures, sprite_sheets): SData| {
            let texture_handle = { 
                loader.load(
                    format!("{}.{}", PROJECTILES_PATH, "png"),
                    ImageFormat::default(),
                    &mut progress,
                    &textures,
                )
            };
            loader.load(
                format!("{}.{}", PROJECTILES_PATH, "ron"),
                SpriteSheetFormat(texture_handle),
                &mut progress,
                &sprite_sheets,
            )
        }));
        progress
    }
}
