use amethyst_core::math::Vector3;
use specs::prelude::*;
use serde_derive::Deserialize;
use std::f32;

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct ArcBallControls {
    /// The point the camera is looking at.
    pub target: Vector3<f32>,
    /// Distance from the camera to the point it is looking at.
    pub distance: f32,
    pub sensitivity_translate: Vector3<f32>,
    pub sensitivity_zoom: f32,
    pub sensitivity_pitch: f32,
    pub sensitivity_yaw: f32,
}

impl Default for ArcBallControls {
    fn default() -> Self {
        Self {
            target: Vector3::new(0.0, 0.0, 0.0),
            distance: 1.0,
            sensitivity_translate: Vector3::new(1.0, 1.0, 1.0),
            sensitivity_zoom: 1.0,
            sensitivity_pitch: 1.0,
            sensitivity_yaw: 1.0,
        }
    }
}

impl Component for ArcBallControls {
    type Storage = VecStorage<Self>;
}
