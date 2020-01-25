use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::core::math::Vector2;
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

use crate::components::{Enemy, Pos, Vel, Waypoints};
use crate::controls::Bindings;

#[derive(Default)]
pub struct SpawnSystem {
    event_reader: Option<ReaderId<InputEvent<Bindings>>>,
    material: Option<Handle<Material>>,
    mesh: Option<Handle<Mesh>>,
}

impl<'s> System<'s> for SpawnSystem {
    type SystemData = (
        WriteStorage<'s, Handle<Material>>,
        WriteStorage<'s, Handle<Mesh>>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, Pos>,
        WriteStorage<'s, Vel>,
        WriteStorage<'s, Waypoints>,
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
            mut materials,
            mut meshes,
            mut transforms,
            mut enemies,
            mut positions,
            mut velocities,
            mut waypoints,
            events,
            entities,
            loader,
            mat_defaults,
            material_assets,
            mesh_assets,
            texture_assets,
        ) = data;

        // Initialization logic is here and not in `setup`,
        // because `MaterialDefaults` is not yet initialized in `setup`
        // and that can't be fixed with dependencies due to
        // `RenderSystem` being a thread local system (thanks OpenGL).
        // A custom `Dispatcher` is needed.
        let mesh = self.mesh.get_or_insert_with(|| {
            loader.load_from_data::<Mesh, _>(
                Shape::Sphere(32, 32)
                    .generate::<Vec<PosNormTex>>(None)
                    .into(),
                (),
                &mesh_assets,
            )
        });

        let material = self.material.get_or_insert_with(|| {
            let albedo = loader.load_from_data(
                TextureBuilder::new()
                    .with_data(vec![Pixel::<_, _, pixel::Srgb>::from(Srgba::new(
                        1.0, 0.0, 1.0, 0.0,
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
                InputEvent::ActionPressed(s) if s == "spawn_enemy" => true,
                _ => false,
            })
            .for_each(|_| {
                entities
                    .build_entity()
                    .with(mesh.clone(), &mut meshes)
                    .with(material.clone(), &mut materials)
                    .with(Transform::default(), &mut transforms)
                    .with(Enemy { speed: 5.0 }, &mut enemies)
                    .with(Pos(Vector2::new(15.0, 15.0)), &mut positions)
                    .with(Vel(Vector2::new(0.0, 0.0)), &mut velocities)
                    .with(
                        Waypoints {
                            goals: vec![
                                Vector2::new(0, 30),
                                Vector2::new(30, 30),
                                Vector2::new(30, 0),
                                Vector2::new(0, 0),
                            ],
                        },
                        &mut waypoints,
                    )
                    .build();
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
