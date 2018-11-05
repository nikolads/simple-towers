use amethyst::assets::{PrefabLoader, PrefabLoaderSystem, RonFormat};
use amethyst::core::timing::Time;
use amethyst::core::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{DrawShaded, PosNormTex};
use amethyst::utils::fps_counter::{FPSCounter, FPSCounterBundle};
use amethyst::utils::scene::BasicScenePrefab;

mod camera;
mod enemy;
mod ground;
mod spawn;

use self::camera::CameraSystem;
use self::enemy::EnemySystem;
use self::spawn::SpawnSystem;

type GamePrefab = BasicScenePrefab<Vec<PosNormTex>>;

#[derive(Default, Debug)]
struct GameState;

impl<'a, 'b> SimpleState<'a, 'b> for GameState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        let handle = data.world.exec(|loader: PrefabLoader<GamePrefab>| {
            loader.load("prefab/scene.ron", RonFormat, (), ())
        });

        data.world.create_entity().with(handle).build();

        {
            use amethyst::core::cgmath::{PerspectiveFov, Quaternion, Rad, Vector3};
            use amethyst::core::Transform;
            use amethyst::renderer::{Camera, Projection};
            use crate::camera::ArcBallControls;

            data.world
                .create_entity()
                .with(Transform {
                    translation: Vector3::new(0.0, 0.0, 0.0),
                    rotation: Quaternion::new(0.870, -0.493, 0.0, 0.0),
                    scale: Vector3::new(1.0, 1.0, 1.0),
                })
                .with(Camera::from(Projection::Perspective(PerspectiveFov {
                    fovy: Rad(1.0471975512),
                    aspect: 1.0,
                    near: 0.1,
                    far: 2000.0,
                })))
                .with(ArcBallControls {
                    target: Vector3::new(15.0, 0.0, 15.0),
                    distance: 20.0,
                    sensitivity_pitch: 1.0,
                    sensitivity_yaw: 1.0,
                    sensitivity_zoom: 10.0,
                    sensitivity_translate: Vector3::new(10.0, 10.0, 10.0),
                })
                .build();
        }

        ground::generate(&mut data.world, 30, 30);
    }

    fn fixed_update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>, StateEvent> {
        let time = data.world.read_resource::<Time>();
        let fps = data.world.read_resource::<FPSCounter>();

        if time.frame_number() % 100 == 0 {
            println!("fps: {}", fps.sampled_fps());
        }

        Trans::None
    }
}

fn main() -> amethyst::Result<()> {
    {
        use amethyst::{LogLevelFilter, LoggerConfig, StdoutLog};

        amethyst::start_logger(LoggerConfig {
            stdout: StdoutLog::Colored,
            level_filter: LogLevelFilter::Warn,
            ..LoggerConfig::default()
        });
    }

    let bindings_path = "config/bindings.ron";
    let display_path = "config/display.ron";

    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(bindings_path)?;

    let data = GameDataBuilder::new()
        .with(PrefabLoaderSystem::<GamePrefab>::default(), "", &[])
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(FPSCounterBundle::default())?
        .with(CameraSystem, "camera", &["input_system"])
        .with(EnemySystem, "", &[])
        .with(SpawnSystem::default(), "spawn", &["input_system"])
        .with_basic_renderer(display_path, DrawShaded::<PosNormTex>::new(), false)?;

    let mut game = Application::new("assets/", GameState::default(), data)?;
    game.run();

    Ok(())
}
