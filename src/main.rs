use amethyst::assets::{PrefabLoader, PrefabLoaderSystem, RonFormat};
use amethyst::core::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{DrawShaded, PosNormTex};
use amethyst::utils::scene::BasicScenePrefab;

mod camera;

use self::camera::CameraSystem;

type GamePrefab = BasicScenePrefab<Vec<PosNormTex>>;

#[derive(Default, Debug)]
struct GameState;

impl<'a, 'b> SimpleState<'a, 'b> for GameState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let handle = data.world.exec(|loader: PrefabLoader<GamePrefab>| {
            loader.load("prefab/scene.ron", RonFormat, (), ())
        });
        
        data.world.create_entity().with(handle).build();
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let bindings_path = "config/bindings.ron";
    let display_path = "config/display.ron";

    let input_bundle = InputBundle::<String, ()>::new()
        .with_bindings_from_file(bindings_path)?;

    let data = GameDataBuilder::new()
        .with(PrefabLoaderSystem::<GamePrefab>::default(), "", &[])
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(CameraSystem, "camera", &["input_system"])
        .with_basic_renderer(display_path, DrawShaded::<PosNormTex>::new(), false)?;

    let mut game = Application::new("assets/", GameState::default(), data)?;
    game.run();

    Ok(())
}
