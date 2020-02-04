use derive_deref::{Deref, DerefMut};
use specs::{Component, VecStorage};

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Enemy;

#[derive(Component, Debug, Deref, DerefMut)]
#[storage(VecStorage)]
pub struct Speed(pub f32);
