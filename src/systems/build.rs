use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::nalgebra::Vector3;
use amethyst::core::Transform;
use amethyst::ecs::prelude::*;
use amethyst::input::InputEvent;
use amethyst::renderer::{
    Material, MaterialDefaults, Mesh, MeshHandle, PosNormTex, Shape, Texture,
};
use amethyst::shrev::EventChannel;

use crate::controls::Action;
use crate::components::Selection;

#[derive(Default)]
pub struct BuildSystem {
    event_reader: Option<ReaderId<InputEvent<Action>>>,
    material: Option<Material>,
    mesh: Option<MeshHandle>,
}

impl<'s> System<'s> for BuildSystem {
    type SystemData = (
        ReadStorage<'s, Selection>,
        WriteStorage<'s, MeshHandle>,
        WriteStorage<'s, Material>,
        WriteStorage<'s, Transform>,
        Read<'s, EventChannel<InputEvent<Action>>>,
        Entities<'s>,
        ReadExpect<'s, Loader>,
        ReadExpect<'s, MaterialDefaults>,
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
            mesh_assets,
            texture_assets,
        ) = data;

        let mesh = self.mesh.get_or_insert_with(|| {
            loader.load_from_data::<Mesh, _>(
                Shape::Cylinder(32, None).generate::<Vec<PosNormTex>>(Some((0.5, 0.5, 1.0))),
                (),
                &mesh_assets,
            )
        });

        let material = self.material.get_or_insert_with(|| {
            let albedo = loader.load_from_data([0.0, 0.0, 1.0, 0.0].into(), (), &texture_assets);

            Material {
                albedo,
                ..mat_defaults.0.clone()
            }
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
                    transform.rotate_global(Vector3::x_axis(), std::f32::consts::FRAC_PI_2);
                    transform.set_y(1.0);

                    entities
                        .build_entity()
                        .with(mesh.clone(), &mut meshes)
                        .with(material.clone(), &mut materials)
                        .with(transform, &mut transforms)
                        .build();
                }
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
