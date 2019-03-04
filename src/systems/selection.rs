use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::nalgebra::{Point3, Vector2, Vector3};
use amethyst::core::Transform;
use amethyst::ecs::prelude::*;
use amethyst::renderer::{
    Camera, Event, Material, MaterialDefaults, Mesh, MeshHandle, PosNormTex, Shape, Texture,
    WindowEvent,
};
use amethyst::shrev::EventChannel;

use crate::components::Selection;
use crate::terrain::Terrain;

#[derive(Default)]
pub struct SelectionSystem {
    event_reader: Option<ReaderId<Event>>,
    material: Option<Material>,
    mesh: Option<MeshHandle>,
}

impl<'s> System<'s> for SelectionSystem {
    type SystemData = (
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Selection>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, MeshHandle>,
        WriteStorage<'s, Material>,
        Read<'s, EventChannel<Event>>,
        Entities<'s>,
        ReadExpect<'s, Loader>,
        ReadExpect<'s, MaterialDefaults>,
        ReadExpect<'s, AssetStorage<Mesh>>,
        ReadExpect<'s, AssetStorage<Texture>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            cams,
            mut selections,
            mut transforms,
            mut meshes,
            mut materials,
            events,
            entities,
            loader,
            mat_defaults,
            mesh_assets,
            texture_assets,
        ) = data;

        let reader = self
            .event_reader
            .as_mut()
            .expect("`SelectSystem::setup` was not called before `SelectSystem::run`");

        for event in events.read(reader) {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CursorMoved { position, .. },
                    ..
                } => {
                    let mut selection = None;

                    if let Some((cam, transf)) = (&cams, &transforms).join().next() {
                        let v = screen_to_world(Into::into(*position), cam, transf);

                        let cam_pos = transf.translation();
                        let dir = v - cam_pos;

                        let t = -cam_pos.y / dir.y;
                        let target = cam_pos + t * dir;

                        selection = Terrain::coord_to_cell(target).map(|s| Selection(s));
                    }

                    // TODO: if selection is none - delete entity
                    if let Some(sel) = selection {
                        let (entity_selection, entity_transform) =
                            // `unwrap_or_else` causes borrowchk problems
                            match (&mut selections, &mut transforms).join().next() {
                                Some(res) => res,
                                None => {
                                    let mesh = self.mesh.get_or_insert_with(|| {
                                        loader.load_from_data::<Mesh, _>(
                                            Shape::Cube
                                                .generate::<Vec<PosNormTex>>(Some((0.4, 0.1, 0.4))),
                                            (),
                                            &mesh_assets,
                                        )
                                    });

                                    let material = self.material.get_or_insert_with(|| {
                                        let albedo = loader.load_from_data(
                                            [1.0, 0.5, 0.0, 0.0].into(),
                                            (),
                                            &texture_assets,
                                        );

                                        Material {
                                            albedo,
                                            ..mat_defaults.0.clone()
                                        }
                                    });

                                    let ent = entities
                                        .build_entity()
                                        .with(Selection(Vector2::new(0, 0)), &mut selections)
                                        .with(Transform::default(), &mut transforms)
                                        .with(mesh.clone(), &mut meshes)
                                        .with(material.clone(), &mut materials)
                                        .build();

                                    (
                                        selections.get_mut(ent.clone()).unwrap(),
                                        transforms.get_mut(ent.clone()).unwrap(),
                                    )
                                },
                            };

                        entity_transform.set_position(sel.position());
                        *entity_selection = sel;
                    }
                },
                _ => (),
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.event_reader = Some(res.fetch_mut::<EventChannel<Event>>().register_reader());
    }
}

fn screen_to_world(mouse: (f64, f64), cam: &Camera, transf: &Transform) -> Vector3<f32> {
    // TODO: see https://github.com/KuSpa/RockRaiders/blob/move_click/src/eventhandling/mouse_ray.rs#L46
    // for alternative solution which gives the direction vector directly

    // FIXME: hardcoded window dimentions (600x600)
    let mut v = Point3::new(
        -1.0 + mouse.0 as f32 / 300.0,
        -(-1.0 + mouse.1 as f32 / 300.0),
        1.0,
    )
    .to_homogeneous();

    v = cam.proj.try_inverse().unwrap() * v;
    v = v / v.w;
    v = transf.matrix() * v;

    Vector3::new(v.x, v.y, v.z)
}
