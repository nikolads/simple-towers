use amethyst_core::math::Vector2;
use amethyst_input::BindingTypes;
use derive_deref::{Deref, DerefMut};
use std::fmt::{self, Debug, Display};

use crate::components::TowerType;

#[derive(Debug, Eq, PartialEq)]
pub struct Bindings;
impl BindingTypes for Bindings {
    type Axis = Axis;
    type Action = Action;
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Axis {}

impl Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Action {
    SpawnEnemy,
    SelectTower(TowerType),
    BuildTower,
}

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct MousePos(pub Option<Vector2<i32>>);
