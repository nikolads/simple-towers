use amethyst::assets::{PrefabData, ProgressCounter};
use amethyst::core::Transform;
use amethyst::derive::PrefabData;
use amethyst::ecs::prelude::*;
use amethyst::error::Error;
use amethyst::renderer::camera::CameraPrefab;
use amethyst::renderer::light::LightPrefab;
use amethyst::renderer::formats::GraphicsPrefab;
use amethyst::renderer::rendy::util::types::vertex::PosNormTex;
use serde::Deserialize;

use crate::components::ArcBallControls;

#[derive(Default, Deserialize, PrefabData)]
#[serde(default)]
pub struct GamePrefab {
    pub arc_ball_controls: Option<ArcBallControls>,
    pub camera: Option<CameraPrefab>,
    pub graphics: Option<GraphicsPrefab<Vec<PosNormTex>>>,
    pub light: Option<LightPrefab>,
    pub transform: Option<Transform>,
}
