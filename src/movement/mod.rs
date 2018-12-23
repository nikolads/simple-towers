use amethyst::core::nalgebra::{UnitQuaternion, Vector2, Vector3};
use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::prelude::*;

mod waypoint;

pub use self::waypoint::{Waypoint, WaypointSystem};

#[derive(Clone, Debug)]
pub struct Movement {
    pub pos: Vector2<f32>,
    pub vel: Vector2<f32>,
}

impl Component for Movement {
    type Storage = VecStorage<Self>;
}

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Movement>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut moves, mut transforms, time) = data;

        for (mv, transform) in (&mut moves, &mut transforms).join() {
            mv.pos += mv.vel * time.delta_seconds();

            let pos_3d = Vector3::new(mv.pos.x, 0.0, mv.pos.y);
            transform.set_position(pos_3d);

            if pos_3d.norm_squared() > 0.0 {
                transform.set_rotation(UnitQuaternion::look_at_rh(&pos_3d, &Vector3::y()));
            }
        }
    }
}
