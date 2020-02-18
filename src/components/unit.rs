use derive_deref::{Deref, DerefMut};
use specs::{Component, NullStorage, VecStorage};

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Enemy;

/// Base speed
#[derive(Component, Debug, Deref, DerefMut)]
#[storage(VecStorage)]
pub struct Speed(pub f32);
