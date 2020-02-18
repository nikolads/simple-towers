use euclid::{Point2D, Rect};
use std::iter;

pub fn rect_points<U>(rect: Rect<usize, U>) -> impl Iterator<Item = Point2D<usize, U>> {
    let xs = rect.x_range();
    let ys = rect.y_range();

    xs.flat_map(move |x| iter::repeat(x).zip(ys.clone()))
        .map(|(x, y)| Point2D::new(x, y))
}
