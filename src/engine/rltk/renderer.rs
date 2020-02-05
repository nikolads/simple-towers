use amethyst_core::math::Vector2;
use object_pool::Reusable;
use rltk::{self, ColorPair, DrawBatch, Point, RGB};
use specs::prelude::*;

use crate::components::{Enemy, Pos, Selection, Tower};
use crate::terrain::{Map, Tile};

pub fn draw_map(batch: &mut Reusable<DrawBatch>, map: &Map) {
    batch.target(0);

    for (tile, coords) in map.tiles() {
        match tile {
            Tile::Grass => {
                batch.set(
                    Point::new(coords.x as i32, coords.y as i32),
                    ColorPair::new(RGB::from_f32(0.0, 0.0, 0.0), RGB::from_f32(0.3, 0.5, 0.0)),
                    rltk::to_cp437(' '),
                );
            }
        }
    }
}

type DrawEnemiesSystemData<'a> = (ReadStorage<'a, Enemy>, ReadStorage<'a, Pos>);
pub fn draw_enemies<'a>(batch: &mut Reusable<DrawBatch>, data: DrawEnemiesSystemData<'a>) {
    let (enemies, positions) = data;
    batch.target(1);

    for (_, pos) in (&enemies, &positions).join() {
        let (x, y) = (pos.0.x as i32, pos.0.y as i32);

        batch.set(
            Point::new(x, y),
            ColorPair::new(RGB::from_f32(1.0, 0.0, 1.0), RGB::from_f32(0.0, 0.0, 0.0)),
            rltk::to_cp437('@'),
        );
    }
}

type DrawBuildingsSystemData<'a> = ReadStorage<'a, Tower>;
pub fn draw_buildings<'a>(batch: &mut Reusable<DrawBatch>, towers: DrawBuildingsSystemData<'a>) {
    batch.target(1);

    for tower in (&towers).join() {
        for tile in tower.position.tiles() {
            let (x, y) = (tile.x as i32, tile.y as i32);

            batch.set(
                Point::new(x, y),
                ColorPair::new(RGB::from_f32(0.0, 0.0, 1.0), RGB::from_f32(0.0, 0.0, 0.0)),
                rltk::to_cp437('#'),
            );
        }
    }
}

type DrawSelectionsSystemData<'a> = ReadExpect<'a, Option<Selection>>;
pub fn draw_selections<'a>(
    batch: &mut Reusable<DrawBatch>,
    selection: DrawSelectionsSystemData<'a>,
) {
    batch.target(0);

    if let Some(sel) = &*selection {
        let mut per_tile = |coords: Vector2<usize>| {
            let (x, y) = (coords.x as i32, coords.y as i32);

            batch.set(
                Point::new(x, y),
                ColorPair::new(RGB::from_f32(0.0, 0.0, 0.0), RGB::from_f32(1.0, 0.3, 1.0)),
                rltk::to_cp437(' '),
            );
        };

        match sel {
            Selection::Hover(coords) => per_tile(*coords),
            Selection::PlaceBuilding(rect) => rect.tiles().for_each(per_tile),
        }
    }
}
