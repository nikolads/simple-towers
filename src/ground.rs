use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::nalgebra::{UnitQuaternion, Vector3};
use amethyst::core::Transform;
use amethyst::ecs::prelude::*;
use amethyst::renderer::{Material, MaterialDefaults, Mesh, PosNormTex, Shape, Texture};
use rand::distributions::{Weighted, WeightedChoice};
use rand::prelude::*;
use rand::rngs::SmallRng;

pub fn generate(world: &mut World, width: usize, height: usize) {
    let mut rng = SmallRng::from_entropy();

    let mesh = {
        let loader = world.read_resource::<Loader>();

        let mesh_assets = world.read_resource::<AssetStorage<Mesh>>();
        let mesh = loader.load_from_data::<Mesh, _>(
            Shape::Cube.generate::<Vec<PosNormTex>>(Some((0.5, 0.5, 0.5))),
            (),
            &mesh_assets,
        );

        mesh
    };

    let materials = {
        let loader = world.read_resource::<Loader>();

        (0..3)
            .map(|i| {
                let texture_assets = world.read_resource::<AssetStorage<Texture>>();
                let albedo = loader.load_from_data(
                    [0.4 - 0.2 * i as f32, 0.6 + 0.2 * i as f32, 0.0, 0.0].into(),
                    (),
                    &texture_assets,
                );

                let mat_defaults = world.read_resource::<MaterialDefaults>();
                let material = Material {
                    albedo,
                    ..mat_defaults.0.clone()
                };

                material
            })
            .collect::<Vec<_>>()
    };

    let weights = &mut [
        Weighted { weight: 10, item: 0 },
        Weighted { weight: 4, item: 1 },
        Weighted { weight: 1, item: 2 },
    ];
    let weights = WeightedChoice::new(weights);

    for x in 0..width {
        for y in 0..height {
            let height = weights.sample(&mut rng);

            let transform = {
                let mut t = Transform::default();
                t.set_position(Vector3::new(
                    x as f32 + 0.5,
                    -1.0 + height as f32 * 0.5,
                    y as f32 + 0.5,
                ));
                t.set_rotation(UnitQuaternion::identity());
                t.set_scale(1.0, height as f32 + 2.0, 1.0);
                t
            };

            world
                .create_entity()
                .with(transform)
                .with(mesh.clone())
                .with(materials[height as usize].clone())
                .build();
        }
    }
}
