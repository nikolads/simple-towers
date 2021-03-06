use amethyst::core::nalgebra::Vector2;
use amethyst::ecs::prelude::*;
use derive_deref::{Deref, DerefMut};

/// Position component.
#[derive(Clone, Debug, Deref, DerefMut)]
pub struct Pos(pub Vector2<f32>);

impl Component for Pos {
    type Storage = VecStorage<Self>;
}

/// Velocity component.
#[derive(Clone, Debug, Deref, DerefMut)]
pub struct Vel(pub Vector2<f32>);

impl Component for Vel {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Clone)]
pub struct MoveOrder {
    goal: Option<Vector2<i32>>,
    segments: Vec<Vector2<i32>>,
}

impl MoveOrder {
    pub fn new(dest: Vector2<i32>) -> Self {
        Self {
            goal: None,
            segments: vec![dest],
        }
    }

    pub fn goal(&self) -> Option<&Vector2<i32>> {
        self.goal.as_ref()
    }

    pub fn next_segment(&mut self) {
        if self.segments.is_empty() {
            self.goal = None;
        } else {
            self.goal = Some(self.segments.remove(0));
        }
    }
}

impl Component for MoveOrder {
    type Storage = VecStorage<Self>;
}

/// A queue of goals to move to.
#[derive(Debug, Clone)]
pub struct Waypoints {
    pub goals: Vec<Vector2<i32>>,
}

impl Component for Waypoints {
    type Storage = VecStorage<Self>;
}
