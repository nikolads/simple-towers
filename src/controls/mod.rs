use amethyst_input::BindingTypes;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Display};

#[derive(Debug, Eq, PartialEq)]
pub struct Bindings;
impl BindingTypes for Bindings {
    type Axis = Axis;
    type Action = Action;
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Axis {}

impl Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Action {
    SpawnEnemy,
    Build,
}
