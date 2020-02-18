use amethyst_input::InputEvent;
use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use specs::shrev::EventChannel;

use crate::components::TowerType;
use crate::controls::{Action, Bindings};

type HandleInputSystemData<'a> = (WriteExpect<'a, EventChannel<InputEvent<Bindings>>>,);
pub fn handle_input<'a>(ctx: &mut Rltk, data: HandleInputSystemData<'a>) {
    let (mut event_channel,) = data;

    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::O => {
                event_channel.single_write(InputEvent::ActionPressed(Action::SpawnEnemy))
            }
            VirtualKeyCode::Q => event_channel.single_write(InputEvent::ActionPressed(
                Action::SelectTower(TowerType::Red),
            )),
            VirtualKeyCode::W => event_channel.single_write(InputEvent::ActionPressed(
                Action::SelectTower(TowerType::Green),
            )),
            VirtualKeyCode::E => event_channel.single_write(InputEvent::ActionPressed(
                Action::SelectTower(TowerType::Blue),
            )),
            VirtualKeyCode::B => {
                event_channel.single_write(InputEvent::ActionPressed(Action::BuildTower))
            }
            _ => (),
        }
    }
}
