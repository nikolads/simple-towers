//! Strongly typed controls

use amethyst::input::BindingTypes;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Display};

pub mod camera;

#[derive(Debug)]
pub struct Bindings;
impl BindingTypes for Bindings {
    type Axis = Axis;
    type Action = Action;
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Axis {
    Camera(camera::AxisControls),
}

impl From<camera::AxisControls> for Axis {
    fn from(ax: camera::AxisControls) -> Self {
        Axis::Camera(ax)
    }
}

impl Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

// TODO: make it an enum like `Axis`
pub type Action = String;
