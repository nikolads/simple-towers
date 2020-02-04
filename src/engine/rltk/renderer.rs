use amethyst_core::math::Vector2;
use rltk::{self, Console as _, Rltk, RGB};
use specs::prelude::*;

use crate::components::{Enemy, Pos, Selection, Tower};
use crate::terrain::{Map, Tile};

pub fn draw_map(ctx: &mut Rltk, map: &Map) {
    for (tile, coords) in map.tiles() {
        match tile {
            Tile::Grass => ctx.set(
                coords.x as i32,
                coords.y as i32,
                RGB::from_f32(0.0, 0.0, 0.0),
                RGB::from_f32(0.4, 0.6, 0.0),
                rltk::to_cp437(' '),
            ),
        }
    }
}

type DrawEnemiesSystemData<'a> = (ReadStorage<'a, Enemy>, ReadStorage<'a, Pos>);
pub fn draw_enemies<'a>(ctx: &mut Rltk, data: DrawEnemiesSystemData<'a>) {
    let (enemies, positions) = data;

    for (_, pos) in (&enemies, &positions).join() {
        let (x, y) = (pos.0.x as i32, pos.0.y as i32);

        ctx.set(
            x,
            y,
            RGB::from_f32(1.0, 0.0, 1.0),
            RGB::from_f32(0.0, 0.0, 0.0),
            rltk::to_cp437('@'),
        );
    }
}

type DrawBuildingsSystemData<'a> = ReadStorage<'a, Tower>;
pub fn draw_buildings<'a>(ctx: &mut Rltk, towers: DrawBuildingsSystemData<'a>) {
    for tower in (&towers).join() {
        for tile in tower.position.tiles() {
            let (x, y) = (tile.x as i32, tile.y as i32);

            ctx.set(
                x,
                y,
                RGB::from_f32(0.0, 0.0, 1.0),
                RGB::from_f32(0.0, 0.0, 0.0),
                rltk::to_cp437('#'),
            );
        }
    }
}

type DrawSelectionsSystemData<'a> = ReadExpect<'a, Option<Selection>>;
pub fn draw_selections<'a>(ctx: &mut Rltk, selection: DrawSelectionsSystemData<'a>) {
    if let Some(sel) = &*selection {
        let mut per_tile = |coords: Vector2<usize>| {
            let (x, y) = (coords.x as i32, coords.y as i32);

            ctx.set(
                x,
                y,
                RGB::from_f32(0.0, 0.0, 0.0),
                RGB::from_f32(1.0, 0.3, 1.0),
                rltk::to_cp437(' '),
            );
        };

        match sel {
            Selection::Hover(coords) => per_tile(*coords),
            Selection::PlaceBuilding(rect) => rect.tiles().for_each(per_tile),
        }
    }
}
