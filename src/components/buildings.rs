use specs::{Component, VecStorage};

use crate::utils::Rect;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Tower {
    pub position: Rect,
}
