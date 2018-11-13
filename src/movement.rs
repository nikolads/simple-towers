use amethyst::core::cgmath::{Quaternion, Vector2, Vector3};
use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::prelude::*;
use derive_deref::{Deref, DerefMut};

#[derive(Clone, Debug, PartialEq, Deref, DerefMut)]
pub struct Position(pub Vector2<f32>);

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Debug, PartialEq, Deref, DerefMut)]
pub struct Velocity(pub Vector2<f32>);

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Position>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (vel, mut pos, mut transform, time) = data;

        for (vel, pos, transform) in (&vel, &mut pos, &mut transform).join() {
            pos.0 += vel.0 * time.delta_seconds();

            transform.translation = Vector3::new(pos.x, 0.0, pos.y);
            transform.rotation =
                Quaternion::from_arc(-Vector3::unit_z(), Vector3::new(vel.x, 0.0, vel.y), None);
        }
    }
}
