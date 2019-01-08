use amethyst::core::nalgebra::{Vector2, Vector3};

mod select;

pub use self::select::{Selection, SelectSystem};

pub struct Terrain {
}

impl Terrain {
    pub fn coord_to_cell(coord: Vector3<f32>) -> Option<Vector2<i32>> {
        Some(Vector2::new((coord.x + 0.5) as i32, (coord.z + 0.5) as i32))
    }

    pub fn cell_center(cell: Vector2<i32>) -> Vector3<f32> {
        Vector3::new(cell.x as f32 + 0.5, 0.0, cell.y as f32 + 0.5)
    }
}
