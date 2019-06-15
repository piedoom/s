use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter, RonFormat},
    prelude::*,
};

use crate::assets::config::GameConfig;
use crate::assets::prefab::{initialize_prefabs, update_prefabs};
use crate::states::main::MainGameState;
/// The hard-coded path of the parent configuration
const CONFIG_PATH: &'static str = "config/config.ron";

/// This initial loadstate will load a bunch of paths that we will use to load further assets.
#[derive(Default)]
pub struct LoadConfigState {
    pub config_handle: Option<Handle<GameConfig>>,
    pub progress: ProgressCounter,
}

impl SimpleState for LoadConfigState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let loader = &data.world.read_resource::<Loader>();
        let config_handle = loader.load(
            CONFIG_PATH,
            RonFormat,
            &mut self.progress,
            &data.world.read_resource::<AssetStorage<GameConfig>>(),
        );

        self.config_handle = Some(config_handle);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress.is_complete() {
            Trans::Switch(Box::new(LoadState {
                config_handle: Some(
                    self.config_handle
                        .take()
                        .expect("Config handle errored during transition."),
                ),
                ..LoadState::default()
            }))
        } else {
            Trans::None
        }
    }
}

#[derive(Default)]
pub struct LoadState {
    pub config_handle: Option<Handle<GameConfig>>,
    pub prefab_progress: Option<ProgressCounter>,
}

impl SimpleState for LoadState {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        // initialize the prefab resource
        self.prefab_progress = Some(initialize_prefabs(
            &mut data.world,
            self.config_handle
                .clone()
                .expect("Could not clone `config_handle` in `LoadState`."),
        ));
        // initialize primitives resource
        crate::assets::Primitives::initialize(&mut data.world);
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
