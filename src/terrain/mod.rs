#[derive(Debug)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Tile>,
}

impl Map {
    pub fn grass(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: vec![Tile::Grass; width * height],
        }
    }

    #[inline]
    fn index_to_coord(&self, index: usize) -> [usize; 2] {
        assert!(index < self.width * self.height);

        [index % self.width, index / self.width]
    }

    pub fn tiles<'a>(&'a self) -> impl Iterator<Item = (Tile, [usize; 2])> + 'a {
        self.tiles
            .iter()
            .enumerate()
            .map(move |(index, &tile)| (tile, self.index_to_coord(index)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Grass,
}
