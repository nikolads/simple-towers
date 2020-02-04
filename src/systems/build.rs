use amethyst_input::InputEvent;
use specs::prelude::*;
use specs::shrev::EventChannel;

use crate::components::Tower;
use crate::controls::{Action, Bindings};
use crate::components::selection::{Selection, SelectionType};

#[derive(Default)]
pub struct BuildSystem {
    event_reader: Option<ReaderId<InputEvent<Bindings>>>,
}

impl<'s> System<'s> for BuildSystem {
    type SystemData = (
        WriteStorage<'s, Tower>,
        ReadExpect<'s, EventChannel<InputEvent<Bindings>>>,
        ReadExpect<'s, Option<Selection>>,
        WriteExpect<'s, SelectionType>,
        Entities<'s>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut towers, events, selection, mut selection_type, entities) = data;

        events
            .read(self.event_reader.as_mut().unwrap())
            .for_each(|evt| match evt {
                InputEvent::ActionPressed(Action::SelectTower) => {
                    *selection_type = SelectionType::PlaceBuilding;
                },
                InputEvent::ActionPressed(Action::BuildTower) => {
                    match &*selection {
                        Some(Selection::PlaceBuilding(rect)) => {
                            entities.build_entity()
                                .with(Tower { position: rect.clone() }, &mut towers)
                                .build();

                            *selection_type = SelectionType::Hover;
                        },
                        Some(_) => panic!("selection is not `PlaceBuilding`"),
                        None => (),
                    }
                }
                _ => (),
            })
    }

    fn setup(&mut self, res: &mut World) {
        Self::SystemData::setup(res);

        self.event_reader = Some(
            res.fetch_mut::<EventChannel<InputEvent<Bindings>>>()
                .register_reader(),
        );
    }
}
