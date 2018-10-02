use amethyst::assets::{PrefabLoader, PrefabLoaderSystem, RonFormat};
use amethyst::core::TransformBundle;
use amethyst::prelude::*;
use amethyst::renderer::{DrawShaded, PosNormTex};
use amethyst::utils::scene::BasicScenePrefab;

type GamePrefab = BasicScenePrefab<Vec<PosNormTex>>;

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

    let path = "resources/display.ron";

    let data = GameDataBuilder::new()
        .with(PrefabLoaderSystem::<GamePrefab>::default(), "", &[])
        .with_bundle(TransformBundle::new())?
        .with_basic_renderer(path, DrawShaded::<PosNormTex>::new(), false)?;

    let mut game = Application::new("assets/", GameState, data)?;
    game.run();

    Ok(())
}
