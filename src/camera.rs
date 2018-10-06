use amethyst::core::cgmath::{Quaternion, Rad, Vector3};
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::renderer::Camera;

pub struct CameraSystem;

impl<'s> System<'s> for CameraSystem {
    type SystemData = (
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<String, ()>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (cams, mut transforms, inputs) = data;

        for (_, transform) in (&cams, &mut transforms).join() {
            let pos = transform.translation;

            let mut r = (pos.x * pos.x + pos.y * pos.y + pos.z * pos.z).sqrt();
            let mut polar = f32::acos(pos.y / r);
            let mut azimuth = f32::atan2(pos.x, pos.z);

            if let Some(horiz) = inputs.axis_value("camera_horiz") {
                azimuth += horiz as f32 * ::std::f32::consts::PI / 60.0;

                azimuth = match azimuth {
                    azimuth if azimuth < 0.0 => ::std::f32::consts::PI * 2.0 + azimuth,
                    azimuth if azimuth > ::std::f32::consts::PI * 2.0 => {
                        azimuth - ::std::f32::consts::PI * 2.0
                    },
                    azimuth => azimuth,
                };
            }

            if let Some(vert) = inputs.axis_value("camera_vert") {
                polar += vert as f32 * ::std::f32::consts::PI / 120.0;

                polar = match polar {
                    polar if polar < 0.0 => 0.01,
                    polar if polar > ::std::f32::consts::PI => ::std::f32::consts::PI - 0.01,
                    polar => polar,
                };
            }

            if let Some(zoom) = inputs.axis_value("camera_zoom") {
                r += zoom as f32 * 5.0 / 60.0;
            }

            transform.translation = Vector3::new(
                r * f32::sin(polar) * f32::sin(azimuth),
                r * f32::cos(polar),
                r * f32::sin(polar) * f32::cos(azimuth),
            );

            transform.rotation = Quaternion::from_sv(1.0, Vector3::new(0.0, 0.0, 0.0));
            transform.rotate_local(Vector3::new(0.0, 1.0, 0.0), Rad(azimuth));
            transform.rotate_local(
                Vector3::new(1.0, 0.0, 0.0),
                Rad(polar - ::std::f32::consts::PI / 2.0),
            );

            // println!(
            //     "Camera coord: r = {}, θ = {}, φ = {}, up = {:?}",
            //     r,
            //     polar / ::std::f32::consts::PI * 180.0,
            //     azimuth / ::std::f32::consts::PI * 180.0,
            //     transform.orientation().up,
            // );
        }
    }
}
