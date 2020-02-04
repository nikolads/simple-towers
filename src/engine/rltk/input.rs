use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use specs::shrev::EventChannel;
use amethyst_input::InputEvent;

use crate::controls::{Action, Bindings};

type HandleInputSystemData<'a> = (WriteExpect<'a, EventChannel::<InputEvent<Bindings>>>,);
pub fn handle_input<'a>(ctx: &mut Rltk, data: HandleInputSystemData<'a>) {
    let (mut event_channel,) = data;

    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::N => event_channel.single_write(InputEvent::ActionPressed(Action::SpawnEnemy)),
            VirtualKeyCode::T => event_channel.single_write(InputEvent::ActionPressed(Action::SelectTower)),
            VirtualKeyCode::Y => event_channel.single_write(InputEvent::ActionPressed(Action::BuildTower)),
            _ => (),
        }
    }
}
