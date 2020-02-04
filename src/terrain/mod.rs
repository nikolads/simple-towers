use amethyst_core::math::Vector2;
use specs::Entity;

#[derive(Debug)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub building_map: MapImpl<BuildTile>,
}

impl Map {
    pub fn grass(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            building_map: MapImpl::new(
                width,
                height,
                vec![
                    BuildTile {
                        background: Tile::Grass,
                        building: None
                    };
                    width * height
                ],
            ),
        }
    }

    pub fn tiles<'a>(&'a self) -> impl Iterator<Item = (Tile, Vector2<usize>)> + 'a {
        self.building_map
            .tiles
            .iter()
            .enumerate()
            .map(move |(index, tile)| (tile.background, self.building_map.index_to_coord(index)))
    }
}

#[derive(Debug)]
pub struct MapImpl<T> {
    pub width: usize,
    pub height: usize,
    tiles: Vec<T>,
}

impl<T> MapImpl<T> {
    fn new(width: usize, height: usize, tiles: Vec<T>) -> Self {
        Self {
            width,
            height,
            tiles,
        }
    }

    fn index_to_coord(&self, index: usize) -> Vector2<usize> {
        assert!(index < self.width * self.height);

        Vector2::new(index % self.width, index / self.width)
    }

    pub fn contains_i32(&self, x: i32, y: i32) -> bool {
        x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height
    }
}

#[derive(Clone, Debug)]
pub struct BuildTile {
    background: Tile,
    building: Option<Entity>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Grass,
}
