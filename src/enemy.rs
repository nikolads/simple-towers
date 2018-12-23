use amethyst::ecs::prelude::*;

#[derive(Debug)]
pub struct Enemy {
    pub speed: f32,
}

impl Component for Enemy {
    type Storage = VecStorage<Self>;
}
