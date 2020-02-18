use specs::{Component, VecStorage};

#[derive(Clone, Component, Debug)]
#[storage(VecStorage)]
pub struct Tower {
    pub ty: TowerType,
}

#[derive(Clone, Component, Debug)]
#[storage(VecStorage)]
pub struct Blueprint {
    pub ty: TowerType,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TowerType {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
pub enum SelectionType {
    Hover,
    PlaceTower(TowerType),
}
