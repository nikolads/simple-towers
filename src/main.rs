use rltk::{RltkBuilder, SimpleConsole};

use simple_towers::State;

fn main() {
    const WIDTH: usize = 80;
    const HEIGHT: usize = 50;

    let mut ctx = RltkBuilder::new()
        .with_dimensions(WIDTH, HEIGHT)
        .with_tile_dimensions(8, 8)
        .with_title("Simple towers: rltk")
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(WIDTH, HEIGHT, "terminal8x8.png")
        .build();

    ctx.register_console_no_bg(SimpleConsole::init(WIDTH as u32, HEIGHT as u32, &ctx.backend), 0);

    let mut game_state = State::new();
    game_state.setup_systems();
    rltk::main_loop(ctx, game_state);
}
