use amethyst::assets::{PrefabData, PrefabError};
use amethyst::core::cgmath::{ElementWise, Rad, Vector3};
use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::prelude::*;
use amethyst::input::InputHandler;
use amethyst::renderer::Camera;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ArcBallControls {
    pub target: Vector3<f32>,
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

impl<'s> PrefabData<'s> for ArcBallControls {
    type SystemData = WriteStorage<'s, ArcBallControls>;
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        controls: &mut Self::SystemData,
        _: &[Entity],
    ) -> Result<Self::Result, PrefabError> {
        controls.insert(entity, self.clone())?;
        Ok(())
    }
}

pub struct CameraSystem;

impl<'s> System<'s> for CameraSystem {
    type SystemData = (
        ReadStorage<'s, Camera>,
        WriteStorage<'s, ArcBallControls>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (cams, mut controls, mut transforms, inputs, timer) = data;

        for (_, arc_ball, transform) in (&cams, &mut controls, &mut transforms).join() {
            let horiz = inputs.axis_value("camera_pitch").unwrap() as f32;
            let vert = inputs.axis_value("camera_yaw").unwrap() as f32;
            let zoom = inputs.axis_value("camera_zoom").unwrap() as f32;
            let translate = Vector3::new(
                inputs.axis_value("camera_translate_x").unwrap() as f32,
                0.0,
                inputs.axis_value("camera_translate_z").unwrap() as f32,
            );
            let dt = timer.delta_seconds();

            transform.pitch_local((Rad(1.0) * vert * arc_ball.sensitivity_pitch * dt).into());
            transform.yaw_global((Rad(1.0) * horiz * arc_ball.sensitivity_yaw * dt).into());

            arc_ball.distance += zoom * arc_ball.sensitivity_zoom * dt;
            arc_ball.target += translate.mul_element_wise(arc_ball.sensitivity_translate) * dt;

            let displacement = transform.rotation * -Vector3::unit_z() * arc_ball.distance;
            transform.translation = arc_ball.target - displacement;

            // let pos = transform.translation;
            // let r = (pos.x * pos.x + pos.y * pos.y + pos.z * pos.z).sqrt();
            // let polar = f32::acos(pos.y / r);
            // let azimuth = f32::atan2(pos.x, pos.z);

            // println!(
            //     "Camera coord: r = {:.3}, θ = {:.3}, φ = {:.3}, up = {:.3?}",
            //     r,
            //     polar / std::f32::consts::PI * 180.0,
            //     azimuth / std::f32::consts::PI * 180.0,
            //     transform.orientation().up,
            // );
        }
    }
}
