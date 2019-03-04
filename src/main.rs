use amethyst::assets::PrefabLoaderSystem;
use amethyst::core::TransformBundle;
use amethyst::prelude::*;
use amethyst::renderer::{DrawShaded, PosNormTex};
use amethyst::utils::fps_counter::FPSCounterBundle;

use simple_towers::controls::InputBundle;
use simple_towers::prefab::GamePrefab;
use simple_towers::systems::{
    BuildSystem, CameraSystem, MovementSystem, SelectionSystem, SpawnSystem, WaypointSystem,
};
use simple_towers::GameState;

fn start_logger() {
    use amethyst::{LogLevelFilter, LoggerConfig, StdoutLog};

    amethyst::start_logger(LoggerConfig {
        stdout: StdoutLog::Plain,
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
        .with(WaypointSystem, "waypoint", &[])
        .with(SelectionSystem::default(), "selection", &[])
        .with(SpawnSystem::default(), "spawn", &["input_system"])
        .with(BuildSystem::default(), "build", &["input_system"])
        .with_basic_renderer(display_path, DrawShaded::<PosNormTex>::new(), false)?;

    let mut game = Application::new("assets/", GameState::default(), data)?;
    game.run();

    Ok(())
}
