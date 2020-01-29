use amethyst_core::math::Vector2;
use derive_deref::{Deref, DerefMut};
use specs::{Component, VecStorage};

#[derive(Clone, Debug, Deref, DerefMut, Component)]
#[storage(VecStorage)]
pub struct Selection(pub Vector2<i32>);

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct MousePos(pub Option<Vector2<i32>>);
