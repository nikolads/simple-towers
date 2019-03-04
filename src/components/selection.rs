use amethyst::core::nalgebra::{UnitQuaternion, Vector2, Vector3};
use amethyst::core::Transform;
use amethyst::ecs::prelude::*;
use derive_deref::{Deref, DerefMut};

use crate::terrain::Terrain;

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct Selection(pub Vector2<i32>);

impl Selection {
    pub fn transform(&self) -> Transform {
        Transform::new(
            self.position().into(),
            UnitQuaternion::identity(),
            Vector3::new(1.0, 1.0, 1.0),
        )
    }

    pub fn position(&self) -> Vector3<f32> {
        Terrain::cell_center(self.0)
    }
}

impl Component for Selection {
    type Storage = VecStorage<Self>;
}
