use rltk::Rltk;

use simple_towers::State;

fn main() {
    let context = Rltk::init_simple8x8(80, 50, "Simple towers: rltk", "resources");
    let mut game_state = State::new();

    game_state.setup_systems();

    rltk::main_loop(context, game_state);
}
