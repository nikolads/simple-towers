use amethyst_core::math::Vector2;
use std::iter;

#[derive(Clone, Debug)]
pub struct Rect {
    pub left: usize,
    pub top: usize,
    pub width: usize,
    pub height: usize,
}

impl Rect {
    pub fn tiles<'a>(&'a self) -> impl Iterator<Item = Vector2<usize>> + 'a {
        let xs = self.left..(self.left + self.width);
        let ys = self.top..(self.top + self.height);

        xs.flat_map(move |x| iter::repeat(x).zip(ys.clone()))
            .map(move |(x, y)| Vector2::new(x, y))
    }
}
