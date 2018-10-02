use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawSprite, Pipeline, RenderBundle, Stage};

struct GameState;

impl<'a, 'b> SimpleState<'a, 'b> for GameState {}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = "resources/display.ron";
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawSprite::new()),
    );

    let data = GameDataBuilder::new()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?;

    let mut game = Application::new("assets/", GameState, data)?;
    game.run();

    Ok(())
}
