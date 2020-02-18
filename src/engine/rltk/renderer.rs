use object_pool::Reusable;
use rltk::{self, ColorPair, DrawBatch, Point, Rect, RGB};
use specs::prelude::*;

use crate::components::{Blueprint, Enemy, Pos, SelectionType, Tower, TowerType};
use crate::map::{Map, MapPos, Tile};
use crate::utils;

type DrawMapSystemData<'a> = ReadExpect<'a, Map>;
pub fn draw_map(batch: &mut Reusable<DrawBatch>, map: DrawMapSystemData) {
    batch.target(0);

    for (tile, coords) in map.build_map.tiles() {
        match tile.background {
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
pub fn draw_enemies(batch: &mut Reusable<DrawBatch>, data: DrawEnemiesSystemData) {
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

type DrawBuildingsSystemData<'a> = (ReadStorage<'a, MapPos>, ReadStorage<'a, Tower>);
pub fn draw_buildings(batch: &mut Reusable<DrawBatch>, data: DrawBuildingsSystemData) {
    let (positions, towers) = data;
    batch.target(1);

    for (pos, tower) in (&positions, &towers).join() {
        let color = match tower.ty {
            TowerType::Red => RGB::from_f32(1.0, 0.0, 0.0),
            TowerType::Green => RGB::from_f32(0.0, 1.0, 0.0),
            TowerType::Blue => RGB::from_f32(0.0, 0.0, 1.0),
        };

        for tile in utils::rect_points(pos.0) {
            let (x, y) = (tile.x as i32, tile.y as i32);

            batch.set(
                Point::new(x, y),
                ColorPair::new(color, RGB::from_f32(0.0, 0.0, 0.0)),
                rltk::to_cp437('#'),
            );
        }
    }
}

type DrawSelectionsSystemData<'a> = (ReadStorage<'a, MapPos>, ReadStorage<'a, Blueprint>);
pub fn draw_selections<'a>(batch: &mut Reusable<DrawBatch>, data: DrawSelectionsSystemData<'a>) {
    let (map_positions, blueprints) = data;
    batch.target(0);

    for (pos, blueprint) in (&map_positions, &blueprints).join() {
        utils::rect_points(pos.0).for_each(|coords| {
            let (x, y) = (coords.x as i32, coords.y as i32);

            let color = match blueprint.ty {
                TowerType::Red => RGB::from_f32(1.0, 0.0, 0.0),
                TowerType::Green => RGB::from_f32(0.0, 1.0, 0.0),
                TowerType::Blue => RGB::from_f32(0.0, 0.0, 1.0),
            };

            batch.set(
                Point::new(x, y),
                ColorPair::new(color, RGB::from_f32(1.0, 0.3, 1.0)),
                rltk::to_cp437('#'),
            );
        });
    }
}

type DrawUiSystemData<'a> = ReadExpect<'a, SelectionType>;
pub fn draw_ui(batch: &mut Reusable<DrawBatch>, selection: DrawUiSystemData) {
    batch.target(1);

    batch.draw_box(
        Rect::with_exact(0, 40, 79, 49),
        ColorPair::new(RGB::from_f32(1.0, 1.0, 1.0), RGB::from_f32(0.0, 0.0, 0.0)),
    );
    batch.bar_vertical(
        Point::new(40, 41),
        8,
        1,
        1,
        ColorPair::new(RGB::from_f32(1.0, 1.0, 1.0), RGB::from_f32(0.0, 0.0, 0.0)),
    );

    // TODO: not quite accurate
    if let SelectionType::PlaceTower(ty) = *selection {
        let text = format!(
            "Place tower: {}",
            match ty {
                TowerType::Red => "Red tower",
                TowerType::Blue => "Blue tower",
                TowerType::Green => "Green tower",
            }
        );

        batch.print_color(
            Point::new(1, 41),
            text,
            ColorPair::new(RGB::from_f32(0.7, 0.6, 0.0), RGB::from_f32(0.0, 0.0, 0.0)),
        );
    }

    batch.print(Point::new(42, 41), "(Q) Red tower");
    batch.print(Point::new(42, 42), "(W) Green tower");
    batch.print(Point::new(42, 43), "(E) Blue tower");

    batch.print(Point::new(61, 41), "(Q) Red tower");
    batch.print(Point::new(61, 42), "(W) Green tower");
    batch.print(Point::new(61, 43), "(E) Blue tower");
}
