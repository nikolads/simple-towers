use amethyst::core::cgmath::{InnerSpace, Vector2};
use amethyst::ecs::prelude::*;

use crate::movement::{Position, Velocity};

#[derive(Debug)]
pub struct Enemy {
    pub speed: f32,
}

impl Component for Enemy {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct MovementOrder {
    pub goal: Vector2<f32>,
}

impl Component for MovementOrder {
    type Storage = VecStorage<Self>;
}

pub struct EnemySystem;

impl<'s> System<'s> for EnemySystem {
    type SystemData = (
        ReadStorage<'s, Enemy>,
        ReadStorage<'s, Position>,
        WriteStorage<'s, MovementOrder>,
        WriteStorage<'s, Velocity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (enemies, pos, mut orders, mut vel) = data;

        for (enemy, pos, order, vel) in (&enemies, &pos, &mut orders, &mut vel).join() {
            let pg = order.goal - pos.0;
            let orientation = pg.dot(vel.0);

            if orientation <= 0.0 {
                let next_goal = if (order.goal.x > 0.0) ^ (order.goal.y > 0.0) {
                    Vector2::new(-order.goal.x, order.goal.y)
                } else {
                    Vector2::new(order.goal.x, -order.goal.y)
                };

                order.goal = next_goal;
                vel.0 = (next_goal - pos.0).normalize() * enemy.speed;
            }
        }
    }
}
