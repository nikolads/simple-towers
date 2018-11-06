use amethyst::core::cgmath::{Vector2, Vector3};
use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Component, Join, Read, ReadStorage, System, VecStorage, WriteStorage};

pub struct Enemy;

impl Component for Enemy {
    type Storage = VecStorage<Self>;
}

pub struct MovementOrder {
    pub speed: f32,
    pub goal: Vector2<i32>,
}

impl Component for MovementOrder {
    type Storage = VecStorage<Self>;
}

pub struct EnemySystem;

impl<'s> System<'s> for EnemySystem {
    type SystemData = (
        ReadStorage<'s, Enemy>,
        WriteStorage<'s, MovementOrder>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        // Just to have something moving

        let (enemies, mut movements, mut transforms, timer) = data;

        for (_, movement, transform) in (&enemies, &mut movements, &mut transforms).join() {
            let pos = Vector2::new(transform.translation.x, transform.translation.z);

            if movement.goal == pos.cast::<i32>().unwrap() {
                movement.goal = if (movement.goal.x > 0) ^ (movement.goal.y > 0) {
                    Vector2::new(-movement.goal.x, movement.goal.y)
                } else {
                    Vector2::new(movement.goal.x, -movement.goal.y)
                };
            }

            let pos_to_goal = movement.goal.cast::<f32>().unwrap() - pos;
            let dir = Vector3::new(pos_to_goal.x, 0.0, pos_to_goal.y);

            transform.move_along_global(dir, movement.speed * timer.delta_seconds() * 3.0);
        }
    }
}
