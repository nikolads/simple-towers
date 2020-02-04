use amethyst_core::math::Vector2;
use derive_deref::{Deref, DerefMut};

use crate::utils::Rect;

#[derive(Debug)]
pub enum Selection {
    Hover(Vector2<usize>),
    PlaceBuilding(Rect),
}

#[derive(Debug)]
pub enum SelectionType {
    Hover,
    PlaceBuilding,
}

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct MousePos(pub Option<Vector2<i32>>);
