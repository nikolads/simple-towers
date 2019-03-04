use amethyst::ecs::prelude::*;
use amethyst::ecs::storage::StorageEntry;

use crate::components::{MoveOrder, Waypoints};

#[derive(Debug, Default)]
pub struct WaypointSystem;

impl<'a> System<'a> for WaypointSystem {
    type SystemData = (WriteStorage<'a, MoveOrder>, WriteStorage<'a, Waypoints>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut move_orders, mut waypoints) = data;

        for (order, wp) in (move_orders.entries(), &mut waypoints).join() {
            match order {
                StorageEntry::Vacant(entry) => {
                    if !wp.goals.is_empty() {
                        entry.insert(MoveOrder::new(wp.goals.remove(0)));
                    }
                },
                StorageEntry::Occupied(_) => {},
            }
        }
    }
}
