use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::nalgebra::{Vector2, Vector3};
use amethyst::core::Transform;
use amethyst::ecs::prelude::*;
use amethyst::renderer::{Material, MaterialDefaults, Mesh, PosNormTex, Shape, Texture};

#[derive(Debug)]
pub struct Terrain {
    cell_entities: Vec<Entity>,
    grid_entities: Vec<Entity>,
}

impl Terrain {
    pub fn generate(world: &mut World, width: usize, height: usize) -> Self {
        Self {
            cell_entities: Self::generate_cells(world, width, height),
            grid_entities: Self::generate_grid(world, width, height),
        }
    }

    pub fn coord_to_cell(coord: Vector3<f32>) -> Option<Vector2<i32>> {
        Some(Vector2::new((coord.x + 0.5) as i32, (coord.z + 0.5) as i32))
    }

    pub fn cell_center(cell: Vector2<i32>) -> Vector3<f32> {
        Vector3::new(cell.x as f32 + 0.5, 0.0, cell.y as f32 + 0.5)
    }

    fn generate_cells(world: &mut World, width: usize, height: usize) -> Vec<Entity> {
        let mesh = {
            let loader = world.read_resource::<Loader>();

            let mesh_assets = world.read_resource::<AssetStorage<Mesh>>();
            let mesh = loader.load_from_data::<Mesh, _>(
                Shape::Plane(None).generate::<Vec<PosNormTex>>(Some((0.5, 0.5, 1.0))),
                (),
                &mesh_assets,
            );

            mesh
        };

        let material = {
            let loader = world.read_resource::<Loader>();

            let texture_assets = world.read_resource::<AssetStorage<Texture>>();
            let albedo = loader.load_from_data([0.4, 0.6, 0.0, 0.0].into(), (), &texture_assets);

            let mat_defaults = world.read_resource::<MaterialDefaults>();

            Material {
                albedo,
                ..mat_defaults.0.clone()
            }
        };

        (0..width)
            .flat_map(|x| (0..height).map(move |y| (x, y)))
            .map(|(x, y)| {
                let transform = {
                    let mut t = Transform::default();
                    t.set_position(Vector3::new(x as f32 + 0.5, 0.0, y as f32 + 0.5));
                    t.rotate_global(-Vector3::x_axis(), std::f32::consts::FRAC_PI_2);
                    t
                };

                world
                    .create_entity()
                    .with(transform)
                    .with(mesh.clone())
                    .with(material.clone())
                    .build()
            })
            .collect::<Vec<_>>()
    }

    fn generate_grid(world: &mut World, width: usize, height: usize) -> Vec<Entity> {
        // TODO: render grid in a seperate render pass with lines instead of triangles

        let mesh = {
            let loader = world.read_resource::<Loader>();

            let mesh_assets = world.read_resource::<AssetStorage<Mesh>>();
            let mesh = loader.load_from_data::<Mesh, _>(
                Shape::Plane(None).generate::<Vec<PosNormTex>>(None),
                (),
                &mesh_assets,
            );

            mesh
        };

        let material = {
            let loader = world.read_resource::<Loader>();

            let texture_assets = world.read_resource::<AssetStorage<Texture>>();
            let albedo = loader.load_from_data([1.0, 0.0, 0.0, 0.0].into(), (), &texture_assets);

            let mat_defaults = world.read_resource::<MaterialDefaults>();

            Material {
                albedo,
                ..mat_defaults.0.clone()
            }
        };


        let mut grid = Vec::new();

        grid.extend((0..width - 1).map(|x| {
            let transform = {
                let mut t = Transform::default();
                t.set_scale(0.1, height as f32 * 0.5, 1.0);
                t.set_position(Vector3::new(x as f32 + 1.0, 0.01, height as f32 * 0.5));
                t.rotate_global(-Vector3::x_axis(), std::f32::consts::FRAC_PI_2);
                t
            };

            world
                .create_entity()
                .with(transform)
                .with(mesh.clone())
                .with(material.clone())
                .build()
        }));

        grid.extend((0..height - 1).map(|y| {
            let transform = {
                let mut t = Transform::default();
                t.set_scale(width as f32 * 0.5, 0.1, 1.0);
                t.set_position(Vector3::new(width as f32 * 0.5, 0.01, y as f32 + 1.0));
                t.rotate_global(-Vector3::x_axis(), std::f32::consts::FRAC_PI_2);
                t
            };

            world
                .create_entity()
                .with(transform)
                .with(mesh.clone())
                .with(material.clone())
                .build()
        }));

        grid
    }
}
