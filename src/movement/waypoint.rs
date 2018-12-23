use amethyst::core::nalgebra::Vector2;
use amethyst::ecs::prelude::*;

use super::Movement;
use crate::enemy::Enemy;

#[derive(Clone, Debug, Default)]
pub struct Waypoint {
    current: Option<Vector2<f32>>,
    stack: Vec<Vector2<f32>>,
}

impl Waypoint {
    pub fn new<I>(goals: I) -> Self
    where
        I: IntoIterator<Item = Vector2<f32>>,
        I::IntoIter: DoubleEndedIterator,
    {
        Waypoint {
            current: None,
            stack: goals.into_iter().rev().collect(),
        }
    }

    fn to_next(&mut self, mv: &mut Movement, enemy: &Enemy) {
        self.current = self.stack.pop();

        match self.current {
            Some(next_goal) => {
                mv.vel = (next_goal - mv.pos).normalize() * enemy.speed;
            },
            None => {
                mv.vel = Vector2::new(0.0, 0.0);
            },
        }
    }
}

impl Component for Waypoint {
    type Storage = VecStorage<Self>;
}

pub struct WaypointSystem;

impl<'s> System<'s> for WaypointSystem {
    type SystemData = (
        ReadStorage<'s, Enemy>,
        WriteStorage<'s, Movement>,
        WriteStorage<'s, Waypoint>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (enemies, mut moves, mut waypoints) = data;

        for (enemy, mv, waypoint) in (&enemies, &mut moves, &mut waypoints).join() {
            match waypoint.current {
                Some(goal) => {
                    let orientation = (goal - mv.pos).dot(&mv.vel);

                    // We are moving away from the goal, so we must have passed it
                    if orientation <= 0.0 {
                        // Set current position to the goal, as if we stopped once we reached it
                        mv.pos = goal.clone();

                        waypoint.to_next(mv, enemy)
                    }
                },
                None => waypoint.to_next(mv, enemy),
            }
        }
    }
}
