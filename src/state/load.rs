use amethyst::{
    prelude::*,
    assets::{
        Handle,
        Loader,
        RonFormat,
        ProgressCounter,
        AssetStorage,
    }
};

use crate::asset::config::GameConfig;

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

    fn update(
        &mut self,
        _data: &mut StateData<'_, GameData<'_, '_>>,
    ) -> SimpleTrans {
        if self.progress.is_complete() {
            Trans::Switch(Box::new(LoadState {
                config_handle: Some(self.config_handle.take().unwrap()),
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
    pub progress: ProgressCounter,
}

impl SimpleState for LoadState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let config_storage = &data.world.read_resource::<AssetStorage<GameConfig>>();
        let config = &config_storage.get(&self.config_handle.clone().unwrap()).unwrap();

        dbg!(config);
    }
}