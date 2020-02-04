use amethyst_core::math::Vector2;
use specs::prelude::*;

use crate::components::selection::{Selection, SelectionType};
use crate::components::MousePos;
use crate::terrain::Map;
use crate::utils::Rect;

/// Handles moving the current selection with the mouse.
#[derive(Default)]
pub struct SelectionSystem {}

impl<'s> System<'s> for SelectionSystem {
    type SystemData = (
        ReadExpect<'s, Map>,
        ReadExpect<'s, MousePos>,
        ReadExpect<'s, SelectionType>,
        WriteExpect<'s, Option<Selection>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (map, mouse, sel_type, mut selection) = data;

        if let Some(mouse_pos) = mouse
            .0
            .filter(|pos| map.building_map.contains_i32(pos.x, pos.y))
        {
            *selection = match *sel_type {
                SelectionType::Hover => Some(Selection::Hover(Vector2::new(
                    mouse_pos.x as usize,
                    mouse_pos.y as usize,
                ))),
                SelectionType::PlaceBuilding => Some(Selection::PlaceBuilding(Rect {
                    left: mouse_pos.x as usize,
                    top: mouse_pos.y as usize,
                    width: 2,
                    height: 2,
                })),
            }
        } else {
            *selection = None;
        }
    }
}
