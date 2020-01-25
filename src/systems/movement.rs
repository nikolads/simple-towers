use amethyst::core::math::{UnitQuaternion, Vector2, Vector3};
use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::prelude::*;
use amethyst::ecs::storage::StorageEntry;

use crate::components::{MoveOrder, Pos, Vel};

#[derive(Default, Debug)]
pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, MoveOrder>,
        WriteStorage<'a, Pos>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Vel>,
        Read<'a, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut move_orders, mut positions, mut transforms, mut velocities, time) = data;

        for (pos, vel, transform) in (&mut positions, &velocities, &mut transforms).join() {
            pos.0 += vel.0 * time.delta_seconds();

            let pos_3d = Vector3::new(pos.x, 0.0, pos.y);
            *transform.translation_mut() = pos_3d;

            if pos_3d.norm_squared() > 0.0 {
                transform.set_rotation(UnitQuaternion::look_at_rh(&pos_3d, &Vector3::y()));
            }
        }

        for (pos, vel, order) in (&positions, &mut velocities, move_orders.entries()).join() {
            match order {
                StorageEntry::Occupied(mut entry) => {
                    let orientation = entry.get().goal().map(|goal| {
                        (Vector2::new(goal.x as f32, goal.y as f32) - pos.0).dot(&vel.0)
                    });

                    if vel.0 == Vector2::new(0.0, 0.0) ||
                        orientation.map(|v| v <= 0.0).unwrap_or(false)
                    {
                        entry.get_mut().next_segment();
                        match entry.get().goal() {
                            Some(next) => {
                                vel.0 = (Vector2::new(next.x as f32, next.y as f32) - pos.0).normalize();
                            },
                            None => {
                                entry.remove();
                                vel.0 = Vector2::new(0.0, 0.0);
                            },
                        }
                    }
                },
                StorageEntry::Vacant(_) => {},
            }
        }
    }
}
