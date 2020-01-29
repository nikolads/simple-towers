use amethyst_core::math::Vector2;
use specs::prelude::*;

use crate::components::{Selection, MousePos};

#[derive(Default)]
pub struct SelectionSystem {}

impl<'s> System<'s> for SelectionSystem {
    type SystemData = (
        WriteStorage<'s, Selection>,
        Entities<'s>,
        ReadExpect<'s, MousePos>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut selections,
            entities,
            mouse,
        ) = data;

        if let Some(mouse_pos) = mouse.0 {
            let sel = match (&mut selections).join().next() {
                Some(sel) => sel,
                None => {
                    let ent = entities
                        .build_entity()
                        .with(Selection(Vector2::new(0, 0)), &mut selections)
                        .build();

                    selections.get_mut(ent).unwrap()
                }
            };

            *sel = Selection(mouse_pos);
        }
        // TODO: if selection is none - delete entity
    }
}
