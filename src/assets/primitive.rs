use amethyst::{
    ecs::prelude::*,
    assets::{
        Handle,
        ProgressCounter,
        AssetLoaderSystemData,
    },
    renderer::{
        palette::{LinSrgba, Srgb},
        mtl::{Material, MaterialDefaults},
        rendy::{
            factory::Factory,
            graph::{
                render::{RenderGroupDesc, SubpassBuilder},
                GraphBuilder,
            },
            hal::{format::Format, image},
            mesh::{Normal, Position, Tangent, TexCoord},
            texture::palette::load_from_linear_rgba,
        },
        shape::Shape,
        types::Texture,
        Mesh,
    }
};
use std::collections::HashMap;

#[derive(Default)]
pub struct Primitives {
    pub meshes: HashMap<String, Handle<Mesh>>,
    pub materials: HashMap<String, Handle<Material>>,
}

impl Primitives {
    pub fn initialize(world: &mut World) {
        let mut p = Primitives::default();
        let sphere = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
            loader.load_from_data(
                Shape::Sphere(32, 32)
                    .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                    .into(),
                (),
            )
        });
        p.meshes.insert(String::from("sphere"), sphere);

        let mat_defaults = world.read_resource::<MaterialDefaults>().0.clone();
        let albedo = world.exec(|loader: AssetLoaderSystemData<'_, Texture>| {
            loader.load_from_data(
                load_from_linear_rgba(LinSrgba::new(1.0, 1.0, 1.0, 1.0)).into(),
                (),
            )
        });

        let mtl = world.exec(
            |(mtl_loader, tex_loader): (
                AssetLoaderSystemData<'_, Material>,
                AssetLoaderSystemData<'_, Texture>,
            )| {
                let metallic_roughness = tex_loader.load_from_data(
                    load_from_linear_rgba(LinSrgba::new(0.0, 0.5, 0.0, 0.0))
                        .into(),
                    (),
                );

                mtl_loader.load_from_data(
                    Material {
                        albedo: albedo.clone(),
                        metallic_roughness,
                        ..mat_defaults.clone()
                    },
                    (),
                )
            },
        );
        p.materials.insert(String::from("default"), mtl);
        world.add_resource(p);
    }
}