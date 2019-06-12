//! Pong Tutorial 2

mod assets;
mod components;
mod states;
mod systems;

use crate::assets::config::GameConfig;
use crate::assets::prefab::EntityPrefabData;
use crate::systems as s;

use amethyst::{
    animation::{
        get_animation_set, AnimationBundle, AnimationCommand, AnimationControlSet, AnimationSet,
        EndControl, VertexSkinningBundle,
    },
    assets::{PrefabLoaderSystem, Processor},
    core::transform::Transform,
    core::TransformBundle,
    ecs::{ReadExpect, Resources, SystemData},
    gltf::GltfSceneLoaderSystem,
    input::InputBundle,
    prelude::*,
    renderer::{
        palette::Srgb,
        pass::{
            DrawDebugLinesDesc, DrawFlat2DDesc, DrawFlat2DTransparentDesc, DrawFlatDesc,
            DrawPbrDesc, DrawPbrTransparentDesc, DrawShadedDesc, DrawSkyboxDesc,
        },
        rendy::{
            factory::Factory,
            graph::{
                present::PresentNode,
                render::{RenderGroupDesc, SubpassBuilder},
                GraphBuilder,
            },
            hal::{
                command::{ClearDepthStencil, ClearValue},
                format::Format,
                image,
            },
        },
        sprite::SpriteSheet,
        sprite_visibility::SpriteVisibilitySortingSystem,
        system::{GraphCreator, RenderingSystem},
        transparent::Transparent,
        types::{Backend, DefaultBackend},
        visibility::VisibilitySortingSystem,
    },
    utils::application_root_dir,
    window::{ScreenDimensions, Window, WindowBundle},
};
use systems::input::GameBindings;
use std::sync::Arc;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_path = application_root_dir()?;
    let (assets_path, config_path) = (
        app_path.join("resources"),
        app_path.join("resources/config"),
    );

    let game_data = GameDataBuilder::default()
        // The WindowBundle provides all the scaffolding for opening a window and drawing to it
        .with_bundle(WindowBundle::from_config_path(
            config_path.join("display.ron"),
        ))?
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
            GltfSceneLoaderSystem::default(),
            "gltf_loader",
            &["prefab_loader"], // This is important so that entity instantiation is performed in a single frame.
        )
        .with_bundle(
            AnimationBundle::<usize, Transform>::new("animation_control", "sampler_interpolation")
                .with_dep(&["gltf_loader"]),
        )?
        .with_bundle(VertexSkinningBundle::new().with_dep(&[
            "transform_system",
            "animation_control",
            "sampler_interpolation",
        ]))?
        .with(
            VisibilitySortingSystem::new(),
            "visibility_system",
            &["transform_system"],
        )
        .with(
            Processor::<SpriteSheet>::new(),
            "sprite_sheet_processor",
            &[],
        )
        .with(
            SpriteVisibilitySortingSystem::new(),
            "sprite_visibility_system",
            &["transform_system"],
        )
        .with(Processor::<GameConfig>::new(), "config_processor", &[])
        .with(s::InputSystem::default(), "game_input_system", &[])
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

#[derive(Default)]
struct Graph {
    dimensions: Option<ScreenDimensions>,
    surface_format: Option<Format>,
    dirty: bool,
}

impl<B: Backend> GraphCreator<B> for Graph {
    fn rebuild(&mut self, res: &Resources) -> bool {
        // Rebuild when dimensions change, but wait until at least two frames have the same.
        let new_dimensions = res.try_fetch::<ScreenDimensions>();
        use std::ops::Deref;
        if self.dimensions.as_ref() != new_dimensions.as_ref().map(|d| d.deref()) {
            self.dirty = true;
            self.dimensions = new_dimensions.map(|d| d.clone());
            return false;
        }
        return self.dirty;
    }

    fn builder(&mut self, factory: &mut Factory<B>, res: &Resources) -> GraphBuilder<B, Resources> {
        self.dirty = false;

        let window = <(ReadExpect<'_, Arc<Window>>)>::fetch(res);

        let surface = factory.create_surface(&window);

        // cache surface format to speed things up
        let surface_format = *self
            .surface_format
            .get_or_insert_with(|| factory.get_surface_format(&surface));

        let dimensions = self.dimensions.as_ref().unwrap();
        let window_kind = image::Kind::D2(
            dbg!(dimensions.width()) as u32,
            dimensions.height() as u32,
            1,
            1,
        );

        let mut graph_builder = GraphBuilder::new();
        let color = graph_builder.create_image(
            window_kind,
            1,
            surface_format,
            Some(ClearValue::Color([0.34, 0.36, 0.52, 1.0].into())),
        );

        let depth = graph_builder.create_image(
            window_kind,
            1,
            Format::D32Sfloat,
            Some(ClearValue::DepthStencil(ClearDepthStencil(1.0, 0))),
        );

        let mut opaque_subpass = SubpassBuilder::new();
        let mut transparent_subpass = SubpassBuilder::new();

        opaque_subpass.add_group(DrawPbrDesc::skinned().builder());
        transparent_subpass.add_group(DrawPbrTransparentDesc::skinned().builder());

        let opaque = graph_builder.add_node(
            opaque_subpass
                .with_group(DrawFlat2DDesc::new().builder())
                .with_group(DrawDebugLinesDesc::new().builder())
                .with_group(
                    DrawSkyboxDesc::with_colors(
                        Srgb::new(0.82, 0.51, 0.50),
                        Srgb::new(0.18, 0.11, 0.85),
                    )
                    .builder(),
                )
                .with_color(color)
                .with_depth_stencil(depth)
                .into_pass(),
        );

        let transparent = graph_builder.add_node(
            transparent_subpass
                .with_group(
                    DrawFlat2DTransparentDesc::default()
                        .builder()
                        .with_dependency(opaque),
                )
                .with_color(color)
                .with_depth_stencil(depth)
                .into_pass(),
        );

        let _present = graph_builder.add_node(
            PresentNode::builder(factory, surface, color)
                .with_dependency(opaque)
                .with_dependency(transparent),
        );

        graph_builder
    }
}
