use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::nalgebra::Vector2;
use amethyst::core::Transform;
use amethyst::ecs::prelude::*;
use amethyst::input::InputEvent;
use amethyst::renderer::{
    Material, MaterialDefaults, Mesh, MeshHandle, PosNormTex, Shape, Texture,
};
use amethyst::shrev::EventChannel;

use crate::controls::Action;
use crate::enemy::Enemy;
use crate::movement::{Movement, Waypoint};

#[derive(Default)]
pub struct SpawnSystem {
    event_reader: Option<ReaderId<InputEvent<Action>>>,
    material: Option<Material>,
    mesh: Option<MeshHandle>,
}

impl<'s> System<'s> for SpawnSystem {
    type SystemData = (
        WriteStorage<'s, MeshHandle>,
        WriteStorage<'s, Material>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, Movement>,
        WriteStorage<'s, Waypoint>,
        Read<'s, EventChannel<InputEvent<Action>>>,
        Entities<'s>,
        ReadExpect<'s, Loader>,
        ReadExpect<'s, MaterialDefaults>,
        ReadExpect<'s, AssetStorage<Mesh>>,
        ReadExpect<'s, AssetStorage<Texture>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut meshes,
            mut materials,
            mut transforms,
            mut enemies,
            mut moves,
            mut waypoints,
            events,
            entities,
            loader,
            mat_defaults,
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
                Shape::Sphere(32, 32).generate::<Vec<PosNormTex>>(None),
                (),
                &mesh_assets,
            )
        });

        let material = self.material.get_or_insert_with(|| {
            let albedo = loader.load_from_data([1.0, 0.0, 1.0, 0.0].into(), (), &texture_assets);

            Material {
                albedo,
                ..mat_defaults.0.clone()
            }
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
                    .with(
                        Movement {
                            pos: Vector2::new(15.0, 15.0),
                            vel: Vector2::new(0.0, 0.0),
                        },
                        &mut moves,
                    )
                    .with(
                        Waypoint::new(vec![
                            Vector2::new(0.0, 30.0),
                            Vector2::new(30.0, 30.0),
                            Vector2::new(30.0, 0.0),
                            Vector2::new(0.0, 0.0),
                        ]),
                        &mut waypoints,
                    )
                    .build();
            });
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        self.event_reader = Some(
            res.fetch_mut::<EventChannel<InputEvent<Action>>>()
                .register_reader(),
        );
    }
}
