use amethyst_core::math::Vector2;
use amethyst_input::InputEvent;
use specs::prelude::*;
use specs::shrev::EventChannel;

use crate::components::{Pos, Selection, Tower};
use crate::controls::{Action, Bindings};

#[derive(Default)]
pub struct BuildSystem {
    event_reader: Option<ReaderId<InputEvent<Bindings>>>,
}

impl<'s> System<'s> for BuildSystem {
    type SystemData = (
        ReadStorage<'s, Selection>,
        WriteStorage<'s, Pos>,
        WriteStorage<'s, Tower>,
        ReadExpect<'s, EventChannel<InputEvent<Bindings>>>,
        Entities<'s>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (selection, mut positions, mut towers, events, entities) = data;

        events
            .read(self.event_reader.as_mut().unwrap())
            .filter(|evt| **evt == InputEvent::ActionPressed(Action::Build))
            .for_each(|_| {
                if let Some(sel) = selection.join().next() {
                    entities
                        .build_entity()
                        .with(
                            Pos(Vector2::new(sel.x as f32, sel.y as f32)),
                            &mut positions,
                        )
                        .with(Tower, &mut towers)
                        .build();
                }
            });
    }

    fn setup(&mut self, res: &mut World) {
        Self::SystemData::setup(res);

        self.event_reader = Some(
            res.fetch_mut::<EventChannel<InputEvent<Bindings>>>()
                .register_reader(),
        );
    }
}
