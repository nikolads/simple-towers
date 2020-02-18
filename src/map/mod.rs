use derive_deref::{Deref, DerefMut};
use euclid::{Point2D, Rect, Size2D};
use specs::Entity;
use specs::{Component, VecStorage};
use std::marker::PhantomData;

use crate::utils;

#[derive(Debug)]
pub struct BuildCoords;

pub type BuildPoint = Point2D<usize, BuildCoords>;
pub type BuildRect = Rect<usize, BuildCoords>;
pub type BuildSize = Size2D<usize, BuildCoords>;

#[derive(Clone, Component, Debug, Deref, DerefMut)]
#[storage(VecStorage)]
pub struct MapPos(pub BuildRect);

#[derive(Clone, Component, Debug, Deref, DerefMut)]
#[storage(VecStorage)]
pub struct MapSize(pub BuildSize);

#[derive(Debug, Deref, DerefMut)]
pub struct BuildMap(MapImpl<BuildTile, BuildCoords>);

impl BuildMap {
    pub fn occupied(&self, rect: BuildRect) -> bool {
        utils::rect_points(rect)
            .map(|point| &self.tiles[self.coord_to_index(point)])
            .any(|tile| tile.building.is_some())
    }
}

#[derive(Debug)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub build_map: BuildMap,
}

impl Map {
    pub fn grass(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            build_map: BuildMap(MapImpl::new(
                width,
                height,
                vec![
                    BuildTile {
                        background: Tile::Grass,
                        building: None
                    };
                    width * height
                ],
            )),
        }
    }
}

#[derive(Debug)]
pub struct MapImpl<T, U> {
    pub width: usize,
    pub height: usize,
    tiles: Vec<T>,
    _unit: PhantomData<U>,
}

impl<T, U> MapImpl<T, U> {
    fn new(width: usize, height: usize, tiles: Vec<T>) -> Self {
        Self {
            width,
            height,
            tiles,
            _unit: PhantomData,
        }
    }

    fn coord_to_index(&self, coord: Point2D<usize, U>) -> usize {
        coord.y * self.width + coord.x
    }

    fn index_to_coord(&self, index: usize) -> Point2D<usize, U> {
        assert!(index < self.width * self.height);

        Point2D::new(index % self.width, index / self.width)
    }

    pub fn at(&self, coord: Point2D<usize, U>) -> &T {
        assert!(self.contains(coord));

        &self.tiles[self.coord_to_index(coord)]
    }

    pub fn at_mut(&mut self, coord: Point2D<usize, U>) -> &mut T {
        assert!(self.contains(coord));

        let index = self.coord_to_index(coord);
        &mut self.tiles[index]
    }

    pub fn contains(&self, coord: Point2D<usize, U>) -> bool {
        Rect::from_size(Size2D::new(self.width, self.height)).contains(coord)
    }

    pub fn contains_rect(&self, rect: &Rect<usize, U>) -> bool {
        Rect::from_size(Size2D::new(self.width, self.height)).contains_rect(rect)
    }

    pub fn tiles<'a>(&'a self) -> impl Iterator<Item = (&T, Point2D<usize, U>)> + 'a {
        self.tiles
            .iter()
            .enumerate()
            .map(move |(index, tile)| (tile, self.index_to_coord(index)))
    }
}

#[derive(Clone, Debug)]
pub struct BuildTile {
    pub background: Tile,
    pub building: Option<Entity>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Grass,
}
