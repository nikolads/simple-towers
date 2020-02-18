use amethyst_input::InputEvent;
use euclid::Size2D;
use specs::prelude::*;
use specs::shrev::EventChannel;

use crate::components::{Blueprint, SelectionType, Tower};
use crate::controls::{Action, Bindings};
use crate::map::{BuildRect, Map, MapPos, MapSize};
use crate::utils;

#[derive(Debug, Default)]
pub struct BuildSystem {
    event_reader: Option<ReaderId<InputEvent<Bindings>>>,
    state: State,
}

// TODO: figure out this and `SelectionType`
#[derive(Debug)]
enum State {
    Idle,
    Placing { blueprint: Entity },
}

impl Default for State {
    fn default() -> Self {
        State::Idle
    }
}

impl<'s> System<'s> for BuildSystem {
    type SystemData = (
        WriteStorage<'s, Blueprint>,
        WriteStorage<'s, FollowMouse>,
        WriteStorage<'s, MapPos>,
        WriteStorage<'s, MapSize>,
        WriteStorage<'s, Tower>,
        ReadExpect<'s, EventChannel<InputEvent<Bindings>>>,
        WriteExpect<'s, Map>,
        WriteExpect<'s, SelectionType>,
        Entities<'s>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut blueprints,
            mut follow_mouse,
            mut map_positions,
            mut map_sizes,
            mut towers,
            events,
            mut map,
            mut selection_type,
            entities,
        ) = data;

        events
            .read(self.event_reader.as_mut().unwrap())
            .for_each(|evt| match evt {
                InputEvent::ActionPressed(Action::SelectTower(ty)) => {
                    *selection_type = SelectionType::PlaceTower(*ty);

                    let ent = entities
                        .build_entity()
                        .with(FollowMouse, &mut follow_mouse)
                        .with(MapSize(Size2D::new(2, 2)), &mut map_sizes)
                        .with(Blueprint { ty: *ty }, &mut blueprints)
                        .build();

                    let new_state = State::Placing { blueprint: ent };

                    match std::mem::replace(&mut self.state, new_state) {
                        State::Idle => (),
                        State::Placing { blueprint: old_ent } => entities.delete(old_ent).unwrap(),
                    }
                }
                InputEvent::ActionPressed(Action::BuildTower) => {
                    *selection_type = SelectionType::Hover;

                    let bp_ent = match self.state {
                        State::Idle => panic!("invalid state `Idle`"),
                        State::Placing { blueprint: ent } => ent,
                    };

                    let tower_type = blueprints.get(bp_ent).map(|bp| bp.ty).unwrap();

                    match map_positions.get(bp_ent).cloned() {
                        Some(MapPos(rect)) => {
                            if map.build_map.occupied(rect) {
                                // TODO: show as error in UI instead
                                panic!("position is occupied")
                            }

                            let ent = entities
                                .build_entity()
                                .with(Tower { ty: tower_type }, &mut towers)
                                .with(MapPos(rect), &mut map_positions)
                                .build();

                            for tile in utils::rect_points(rect) {
                                map.build_map.at_mut(tile).building = Some(ent);
                            }

                            entities.delete(bp_ent).unwrap();
                            self.state = State::Idle;
                        }
                        None => {
                            // TODO: show as error in UI instead
                            panic!("invalid placement position");
                        }
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

use euclid::Point2D;
use specs::storage::StorageEntry;
use specs::Component;

use crate::controls::MousePos;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct FollowMouse;

#[derive(Default)]
pub struct BuildingPlacementSystem {}

impl<'s> System<'s> for BuildingPlacementSystem {
    type SystemData = (
        ReadStorage<'s, FollowMouse>,
        ReadStorage<'s, MapSize>,
        WriteStorage<'s, MapPos>,
        ReadExpect<'s, Map>,
        ReadExpect<'s, MousePos>,
        Entities<'s>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (follow_mouse, sizes, mut map_positions, map, mouse, entities) = data;

        if let Some(mouse_pos) = **mouse {
            for (_, size, ent) in (&follow_mouse, &sizes, &entities).join() {
                let target_rect = BuildRect::new(
                    Point2D::new(mouse_pos.x as usize, mouse_pos.y as usize),
                    size.0,
                );

                match map_positions.entry(ent).unwrap() {
                    StorageEntry::Occupied(mut occupied) => {
                        if map.build_map.contains_rect(&target_rect) {
                            occupied.insert(MapPos(target_rect));
                        } else {
                            occupied.remove();
                        }
                    }
                    StorageEntry::Vacant(vacant) => {
                        if map.build_map.contains_rect(&target_rect) {
                            vacant.insert(MapPos(target_rect));
                        }
                    }
                }
            }
        }
    }
}
