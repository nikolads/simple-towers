use amethyst::core::nalgebra::{UnitQuaternion, Vector3};
use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::prelude::*;
use amethyst::renderer::Camera;
use std::f32;

use crate::controls::InputHandler;
use crate::controls::camera::AxisControls;
use crate::components::ArcBallControls;

pub struct CameraSystem;

impl<'s> System<'s> for CameraSystem {
    type SystemData = (
        ReadStorage<'s, Camera>,
        WriteStorage<'s, ArcBallControls>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler>,
        Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (cams, mut controls, mut transforms, inputs, timer) = data;

        for (_, arc_ball, transform) in (&cams, &mut controls, &mut transforms).join() {
            let horiz = inputs
                .axis_value(&AxisControls::RotateHoriz.into())
                .unwrap() as f32;
            let vert = inputs.axis_value(&AxisControls::RotateVert.into()).unwrap() as f32;
            let zoom = inputs.axis_value(&AxisControls::Zoom.into()).unwrap() as f32;
            let translate = Vector3::new(
                inputs.axis_value(&AxisControls::TranslateX.into()).unwrap() as f32,
                0.0,
                inputs.axis_value(&AxisControls::TranslateZ.into()).unwrap() as f32,
            );
            let dt = timer.delta_seconds();

            transform.pitch_local(vert * arc_ball.sensitivity_pitch * dt);
            transform.yaw_global(horiz * arc_ball.sensitivity_yaw * dt);

            arc_ball.distance += zoom * arc_ball.sensitivity_zoom * dt;

            let reverse_y = UnitQuaternion::rotation_between(
                &(transform.rotation() * Vector3::y_axis()),
                &Vector3::y_axis(),
            )
            .unwrap_or(UnitQuaternion::from_axis_angle(
                &Vector3::x_axis(),
                f32::consts::FRAC_PI_2,
            ));

            arc_ball.target += (reverse_y * transform.rotation() * translate)
                .component_mul(&arc_ball.sensitivity_translate) *
                dt;

            let offset_from_target =
                transform.rotation() * -Vector3::z_axis().into_inner() * arc_ball.distance;

            transform.set_position(arc_ball.target - offset_from_target);
        }
    }
}
