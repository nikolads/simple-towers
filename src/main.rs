use amethyst::assets::{PrefabLoader, PrefabLoaderSystem, RonFormat};
use amethyst::core::timing::Time;
use amethyst::core::TransformBundle;
use amethyst::prelude::*;
use amethyst::renderer::{DrawShaded, PosNormTex};
use amethyst::utils::fps_counter::{FPSCounter, FPSCounterBundle};

pub mod camera;
pub mod controls;
pub mod enemy;
pub mod ground;
pub mod movement;
pub mod prefab;
pub mod spawn;

use self::camera::CameraSystem;
use self::controls::InputBundle;
use self::enemy::EnemySystem;
use self::movement::MovementSystem;
use self::prefab::GamePrefab;
use self::spawn::SpawnSystem;

#[derive(Default, Debug)]
struct GameState;

impl<'a, 'b> SimpleState<'a, 'b> for GameState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        let handle = data.world.exec(|loader: PrefabLoader<GamePrefab>| {
            loader.load("prefab/scene.ron", RonFormat, (), ())
        });

        data.world.create_entity().with(handle).build();

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

fn start_logger() {
    use amethyst::{LogLevelFilter, LoggerConfig, StdoutLog};

    amethyst::start_logger(LoggerConfig {
        stdout: StdoutLog::Colored,
        level_filter: LogLevelFilter::Warn,
        ..LoggerConfig::default()
    });
}

fn main() -> amethyst::Result<()> {
    start_logger();

    let bindings_path = "config/bindings.ron";
    let display_path = "config/display.ron";

    let input_bundle = InputBundle::new().with_bindings_from_file(bindings_path)?;

    let data = GameDataBuilder::new()
        .with(PrefabLoaderSystem::<GamePrefab>::default(), "", &[])
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(FPSCounterBundle::default())?
        .with(CameraSystem, "camera", &["input_system"])
        .with(MovementSystem, "movement", &[])
        .with(EnemySystem, "enemy", &["movement"])
        .with(SpawnSystem::default(), "spawn", &["input_system"])
        .with_basic_renderer(display_path, DrawShaded::<PosNormTex>::new(), false)?;

    let mut game = Application::new("assets/", GameState::default(), data)?;
    game.run();

    Ok(())
}
