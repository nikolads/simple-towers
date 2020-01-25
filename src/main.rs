use amethyst::assets::PrefabLoaderSystemDesc;
use amethyst::core::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::bundle::RenderingBundle;
use amethyst::renderer::plugins::{RenderShaded3D, RenderToWindow};
use amethyst::renderer::types::DefaultBackend;
use amethyst::utils::fps_counter::FpsCounterBundle;

use simple_towers::controls::Bindings;
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

    let input_bundle = InputBundle::<Bindings>::new().with_bindings_from_file(bindings_path)?;

    let data = GameDataBuilder::new()
        .with_system_desc(PrefabLoaderSystemDesc::<GamePrefab>::default(), "", &[])
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(FpsCounterBundle::default())?
        .with(CameraSystem, "camera", &["input_system"])
        .with(MovementSystem, "movement", &[])
        .with(WaypointSystem, "waypoint", &[])
        .with(SelectionSystem::default(), "selection", &[])
        .with(SpawnSystem::default(), "spawn", &["input_system"])
        .with(BuildSystem::default(), "build", &["input_system"])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderToWindow::from_config_path(display_path))
                .with_plugin(RenderShaded3D::default()),
        )?;

    let mut game = Application::new("assets/", GameState::default(), data)?;
    game.run();

    Ok(())
}
