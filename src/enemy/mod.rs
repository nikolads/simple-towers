use amethyst::ecs::prelude::*;

mod spawn;

pub use self::spawn::SpawnSystem;

#[derive(Debug)]
pub struct Enemy {
    pub speed: f32,
}

impl Component for Enemy {
    type Storage = VecStorage<Self>;
}
