use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::core::Transform;
use amethyst::ecs::prelude::*;
use amethyst::input::InputEvent;
use amethyst::renderer::palette::rgb::Srgba;
use amethyst::renderer::rendy::texture::pixel::{self, Pixel};
use amethyst::renderer::rendy::texture::TextureBuilder;
use amethyst::renderer::rendy::util::types::vertex::PosNormTex;
use amethyst::renderer::shape::Shape;
use amethyst::renderer::{Material, MaterialDefaults, Mesh, Texture};
use amethyst::shrev::EventChannel;

use crate::components::Selection;
use crate::controls::Bindings;

#[derive(Default)]
pub struct BuildSystem {
    event_reader: Option<ReaderId<InputEvent<Bindings>>>,
    material: Option<Handle<Material>>,
    mesh: Option<Handle<Mesh>>,
}

impl<'s> System<'s> for BuildSystem {
    type SystemData = (
        ReadStorage<'s, Selection>,
        WriteStorage<'s, Handle<Mesh>>,
        WriteStorage<'s, Handle<Material>>,
        WriteStorage<'s, Transform>,
        Read<'s, EventChannel<InputEvent<Bindings>>>,
        Entities<'s>,
        ReadExpect<'s, Loader>,
        ReadExpect<'s, MaterialDefaults>,
        ReadExpect<'s, AssetStorage<Material>>,
        ReadExpect<'s, AssetStorage<Mesh>>,
        ReadExpect<'s, AssetStorage<Texture>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            selection,
            mut meshes,
            mut materials,
            mut transforms,
            events,
            entities,
            loader,
            mat_defaults,
            material_assets,
            mesh_assets,
            texture_assets,
        ) = data;

        let mesh = self.mesh.get_or_insert_with(|| {
            loader.load_from_data::<Mesh, _>(
                Shape::Cylinder(32, None)
                    .generate::<Vec<PosNormTex>>(Some((0.5, 0.5, 1.0)))
                    .into(),
                (),
                &mesh_assets,
            )
        });

        let material = self.material.get_or_insert_with(|| {
            let albedo = loader.load_from_data(
                TextureBuilder::new()
                    .with_data(vec![Pixel::<_, _, pixel::Srgb>::from(Srgba::new(
                        0.0, 0.0, 1.0, 0.0,
                    ))])
                    .into(),
                (),
                &texture_assets,
            );

            loader.load_from_data::<Material, _>(
                Material {
                    albedo,
                    ..mat_defaults.0.clone()
                },
                (),
                &material_assets,
            )
        });

        events
            .read(self.event_reader.as_mut().unwrap())
            .filter(|evt| match evt {
                InputEvent::ActionPressed(s) if s == "build_tower" => true,
                _ => false,
            })
            .for_each(|_| {
                if let Some(sel) = selection.join().next() {
                    let mut transform = sel.transform();
                    transform.prepend_rotation_x_axis(std::f32::consts::FRAC_PI_2);
                    transform.translation_mut().y = 1.0;

                    entities
                        .build_entity()
                        .with(mesh.clone(), &mut meshes)
                        .with(material.clone(), &mut materials)
                        .with(transform, &mut transforms)
                        .build();
                }
            });
    }

    fn setup(&mut self, res: &mut World) {
        Self::SystemData::setup(res);

        self.event_reader = Some(
            res.fetch_mut::<EventChannel<InputEvent<Bindings>>>()
                .register_reader(),
        );
    }
}
