//! Strongly typed controls

use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Display};

pub mod camera;

/// Alias for an amethyst input bundle with controls from this module.
pub type InputBundle = amethyst::input::InputBundle<Axis, Action>;

/// Alias for an amehyst input handler with controls from this module.
pub type InputHandler = amethyst::input::InputHandler<Axis, Action>;

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
