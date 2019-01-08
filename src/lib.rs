use amethyst::assets::{PrefabLoader, RonFormat};
use amethyst::core::timing::Time;
use amethyst::prelude::*;
use amethyst::utils::fps_counter::{FPSCounter};

pub mod camera;
pub mod controls;
pub mod enemy;
pub mod ground;
pub mod movement;
pub mod prefab;
pub mod terrain;

use self::prefab::GamePrefab;

#[derive(Default, Debug)]
pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        let handle = data.world.exec(|loader: PrefabLoader<GamePrefab>| {
            loader.load("prefab/scene.ron", RonFormat, (), ())
        });

        data.world.create_entity().with(handle).build();

        ground::generate(&mut data.world, 30, 30);
    }

    fn fixed_update(&mut self, data: StateData<GameData>) -> Trans<GameData<'static, 'static>, StateEvent> {
        let time = data.world.read_resource::<Time>();
        let fps = data.world.read_resource::<FPSCounter>();

        if time.frame_number() % 100 == 0 {
            println!("fps: {}", fps.sampled_fps());
        }

        Trans::None
    }
}
