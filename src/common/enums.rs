use serde::Deserialize;

#[derive(Clone, Copy,Debug, Deserialize, Hash, Eq, PartialEq)]
pub enum Goods {
    Food,
    Energy,
    Ore,
    Tools
}

#[derive(Clone, Copy, Deserialize, Hash, Eq, PartialEq)]
pub enum TileKind {
    Bush,
    Forest,
    Plains,
    Hills,
    Forge
}

