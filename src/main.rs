//! Main game entrypoint

mod assets;
mod components;
mod render;
mod states;
mod systems;

use crate::assets::config::GameConfig;
use crate::assets::prefab::EntityPrefabData;
use crate::render::Graph;
use crate::systems as s;

use amethyst::{
    assets::{PrefabLoaderSystem, Processor},
    core::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{
        sprite::SpriteSheet, sprite_visibility::SpriteVisibilitySortingSystem,
        system::RenderingSystem, types::DefaultBackend, visibility::VisibilitySortingSystem,
    },
    utils::application_root_dir,
    window::WindowBundle,
};
use systems::input::GameBindings;

fn main() -> amethyst::Result<()> {
     amethyst::Logger::from_config(amethyst::LoggerConfig {
        log_file: Some("s.log".into()),
        level_filter: amethyst::LogLevelFilter::Error,
        ..Default::default()
    }).start();

    let app_path = application_root_dir()?;
    let (assets_path, config_path) = (
        app_path.join("resources"),
        app_path.join("resources/config"),
    );

    let game_data = GameDataBuilder::default()
        // The WindowBundle provides all the scaffolding for opening a window and drawing to it
        .with_bundle(WindowBundle::from_config_path(config_path.join("display.ron")))?
        // Add the transform bundle which handles tracking entity positions
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<GameBindings>::new()
                .with_bindings_from_file(config_path.join("bindings.ron"))?,
        )?
        .with(
            PrefabLoaderSystem::<EntityPrefabData>::default(),
            "prefab_loader",
            &[],
        )
        .with(
            Processor::<SpriteSheet>::new(),
            "sprite_sheet_processor",
            &[],
        )
        .with(Processor::<GameConfig>::new(), "config_processor", &[])
        // Custom systems
        .with(s::InputSystem::default(), "game_input_system", &[])
        .with(
            s::WeaponSystem::default(),
            "weapon_system",
            &["transform_system"],
        )
        .with(
            s::ControllerSystem::default(),
            "controller_system",
            &["game_input_system"],
        )
        // The renderer must be executed on the same thread consecutively, so we initialize it as thread_local
        // which will always execute on the main thread.
        .with_thread_local(RenderingSystem::<DefaultBackend, _>::new(Graph::default()));

    let mut game = Application::new(
        assets_path,
        states::load::LoadConfigState::default(),
        game_data,
    )?;
    game.run();
    Ok(())
}
