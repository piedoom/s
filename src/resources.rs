use amethyst::{
    assets::{
        Asset, AssetStorage, Handle, Loader, Prefab, PrefabData, PrefabLoader, Progress,
        ProgressCounter, RonFormat,
    },
    core::{
        ecs::{Entity, Read, ReadExpect},
        Named, Transform,
    },
    derive::PrefabData,
    prelude::*,
    renderer::{
        camera::CameraPrefab,
        formats::texture::ImageFormat,
        light::LightPrefab,
        sprite::{SpriteSheet, SpriteSheetFormat},
        transparent::Transparent,
        Texture,
    },
    utils::application_root_dir,
    Error,
};

use std::fs::read_dir;

use crate::components as c;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Clone)]
pub struct SResource<V: Asset>(HashMap<String, Handle<V>>);

impl<V> SResource<V>
where
    V: Asset,
{
    pub fn insert(&mut self, ident: String, handle: Handle<V>) -> Option<Handle<V>> {
        self.0.insert(ident, handle)
    }

    pub fn get(&self, ident: String) -> Option<&Handle<V>> {
        self.0.get(&ident)
    }

    pub fn new(data: HashMap<String, Handle<V>>) -> Self {
        Self { 0: data }
    }
}

impl<V> Default for SResource<V>
where
    V: Asset,
{
    fn default() -> Self {
        Self { 0: HashMap::new() }
    }
}

impl<V> From<HashMap<String, Handle<V>>> for SResource<V>
where
    V: Asset,
{
    fn from(data: HashMap<String, Handle<V>>) -> Self {
        SResource::new(data)
    }
}

unsafe impl<V> Send for SResource<V> where V: Asset {}
unsafe impl<V> Sync for SResource<V> where V: Asset {}
