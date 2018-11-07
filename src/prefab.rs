use amethyst::assets::{PrefabData, PrefabError, ProgressCounter};
use amethyst::core::Transform;
use amethyst::derive::PrefabData;
use amethyst::ecs::prelude::*;
use amethyst::renderer::{
    CameraPrefab, GraphicsPrefab, LightPrefab, ObjFormat, PosNormTex, TextureFormat,
};
use serde::Deserialize;

use crate::camera::ArcBallControls;

#[derive(Default, Deserialize, PrefabData)]
#[serde(default)]
pub struct GamePrefab {
    pub arc_ball_controls: Option<ArcBallControls>,
    pub camera: Option<CameraPrefab>,
    pub graphics: Option<GraphicsPrefab<Vec<PosNormTex>, ObjFormat, TextureFormat>>,
    pub light: Option<LightPrefab>,
    pub transform: Option<Transform>,
}
