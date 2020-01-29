use specs::prelude::*;
use rltk::{self, Console as _, Rltk, RGB};

use crate::components::{Enemy, Pos, Tower, Selection};
use crate::terrain::{Map, Tile};

pub fn draw_map(ctx: &mut Rltk, map: &Map) {
    for (tile, [x, y]) in map.tiles() {
        match tile {
            Tile::Grass => ctx.set(
                x as i32,
                y as i32,
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

type DrawBuildingsSystemData<'a> = (ReadStorage<'a, Tower>, ReadStorage<'a, Pos>);
pub fn draw_buildings<'a>(ctx: &mut Rltk, data: DrawBuildingsSystemData<'a>) {
    let (towers, positions) = data;

    for(_, pos) in (&towers, &positions).join() {
        let (x, y) = (pos.0.x as i32, pos.0.y as i32);

        ctx.set(
            x,
            y,
            RGB::from_f32(0.0, 0.0, 1.0),
            RGB::from_f32(0.0, 0.0, 0.0),
            rltk::to_cp437('#'),
        );
    }
}

type DrawSelectionsSystemData<'a> = ReadStorage<'a, Selection>;
pub fn draw_selections<'a>(ctx: &mut Rltk, selections: DrawSelectionsSystemData<'a>) {
    for sel in (&selections).join() {
        let (x, y) = (sel.0.x, sel.0.y);

        ctx.set(
            x,
            y,
            RGB::from_f32(0.0, 0.0, 0.0),
            RGB::from_f32(1.0, 0.3, 1.0),
            rltk::to_cp437(' '),
        );
    }
}
