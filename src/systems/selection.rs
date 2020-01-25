use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::core::math::{Point3, Vector2, Vector3};
use amethyst::core::Transform;
use amethyst::ecs::prelude::*;
use amethyst::renderer::palette::rgb::Srgba;
use amethyst::renderer::rendy::texture::pixel::{self, Pixel};
use amethyst::renderer::rendy::texture::TextureBuilder;
use amethyst::renderer::rendy::util::types::vertex::PosNormTex;
use amethyst::renderer::rendy::wsi::winit::{Event, WindowEvent};
use amethyst::renderer::shape::Shape;
use amethyst::renderer::{Camera, Material, MaterialDefaults, Mesh, Texture};
use amethyst::shrev::EventChannel;

use crate::components::Selection;
use crate::terrain::Terrain;

#[derive(Default)]
pub struct SelectionSystem {
    event_reader: Option<ReaderId<Event>>,
    material: Option<Handle<Material>>,
    mesh: Option<Handle<Mesh>>,
}

impl<'s> System<'s> for SelectionSystem {
    type SystemData = (
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Selection>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Handle<Material>>,
        WriteStorage<'s, Handle<Mesh>>,
        Read<'s, EventChannel<Event>>,
        Entities<'s>,
        ReadExpect<'s, Loader>,
        ReadExpect<'s, MaterialDefaults>,
        ReadExpect<'s, AssetStorage<Material>>,
        ReadExpect<'s, AssetStorage<Mesh>>,
        ReadExpect<'s, AssetStorage<Texture>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            cams,
            mut selections,
            mut transforms,
            mut materials,
            mut meshes,
            events,
            entities,
            loader,
            mat_defaults,
            material_assets,
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
                                                .generate::<Vec<PosNormTex>>(Some((0.4, 0.1, 0.4)))
                                                .into(),
                                            (),
                                            &mesh_assets,
                                        )
                                    });

                                    let material = self.material.get_or_insert_with(|| {
                                        let albedo = loader.load_from_data(
                                            TextureBuilder::new()
                                                .with_data(vec![Pixel::<_, _, pixel::Srgb>::from(Srgba::new(
                                                    1.0, 0.5, 0.0, 0.0,
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

                        *entity_transform.translation_mut() = sel.position();
                        *entity_selection = sel;
                    }
                }
                _ => (),
            }
        }
    }

    fn setup(&mut self, res: &mut World) {
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

    v = cam.as_inverse_matrix() * v;
    v = v / v.w;
    v = transf.matrix() * v;

    Vector3::new(v.x, v.y, v.z)
}
