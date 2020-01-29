use amethyst_core::math::Vector2;
use amethyst_input::InputEvent;
use specs::prelude::*;
use specs::shrev::EventChannel;

use crate::controls::{Action, Bindings};
use crate::components::{Enemy, Pos, Vel, Waypoints};

#[derive(Default)]
pub struct SpawnSystem {
    event_reader: Option<ReaderId<InputEvent<Bindings>>>,
}

impl<'s> System<'s> for SpawnSystem {
    type SystemData = (
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, Pos>,
        WriteStorage<'s, Vel>,
        WriteStorage<'s, Waypoints>,
        ReadExpect<'s, EventChannel<InputEvent<Bindings>>>,
        Entities<'s>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut enemies,
            mut positions,
            mut velocities,
            mut waypoints,
            events,
            entities,
        ) = data;

        events
            .read(self.event_reader.as_mut().unwrap())
            .filter(|evt| **evt == InputEvent::ActionPressed(Action::SpawnEnemy))
            .for_each(|_| {
                entities
                    .build_entity()
                    .with(Enemy { speed: 5.0 }, &mut enemies)
                    .with(Pos(Vector2::new(15.0, 15.0)), &mut positions)
                    .with(Vel(Vector2::new(0.0, 0.0)), &mut velocities)
                    .with(
                        Waypoints {
                            goals: vec![
                                Vector2::new(1, 29),
                                Vector2::new(29, 29),
                                Vector2::new(29, 1),
                                Vector2::new(1, 1),
                            ],
                        },
                        &mut waypoints,
                    )
                    .build();
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
