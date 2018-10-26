use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::cgmath::Vector2;
use amethyst::core::Transform;
use amethyst::ecs::prelude::*;
use amethyst::input::InputHandler;
use amethyst::renderer::{
    Material, MaterialDefaults, Mesh, MeshHandle, PosNormTex, Shape, Texture,
};

use crate::enemy::{Enemy, MovementOrder};

#[derive(Debug, Default)]
pub struct SpawnSystem {
    // FIXME: how to handle events in amethyst?
    last_pressed: bool,
}

impl<'s> System<'s> for SpawnSystem {
    type SystemData = (
        WriteStorage<'s, MeshHandle>,
        WriteStorage<'s, Material>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, MovementOrder>,
        Read<'s, InputHandler<String, String>>,
        ReadExpect<'s, AssetStorage<Mesh>>,
        ReadExpect<'s, AssetStorage<Texture>>,
        ReadExpect<'s, Loader>,
        ReadExpect<'s, MaterialDefaults>,
        Entities<'s>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut meshes,
            mut materials,
            mut transforms,
            mut enemies,
            mut movement_orders,
            input,
            mesh_assets,
            tex_assets,
            loader,
            mat_defaults,
            entities,
        ) = data;

        if let Some(state) = input.action_is_down("spawn_enemy") {
            if state == true && self.last_pressed == false {
                let (mesh, mat) = {
                    let shape = Shape::Sphere(32, 32).generate::<Vec<PosNormTex>>(None);
                    let mesh = loader.load_from_data::<Mesh, _>(shape, (), &mesh_assets);

                    let albedo = loader.load_from_data([1.0, 0.0, 1.0, 0.0].into(), (), &tex_assets);
                    let mat = Material {
                        albedo,
                        ..mat_defaults.0.clone()
                    };

                    (mesh, mat)
                };

                entities
                    .build_entity()
                    .with(mesh, &mut meshes)
                    .with(mat, &mut materials)
                    .with(Transform::default(), &mut transforms)
                    .with(Enemy, &mut enemies)
                    .with(
                        MovementOrder {
                            speed: 2.0,
                            goal: Vector2::new(30, 30),
                        },
                        &mut movement_orders,
                    )
                    .build();
            }

            self.last_pressed = state;
        }
    }
}
