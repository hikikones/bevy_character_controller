use bevy::prelude::*;

use bevy_grid::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Board::new());
    }
}

#[derive(Deref, DerefMut)]
pub struct Board(GridInfinite<SquareCell, Tile>);

impl Board {
    pub fn new() -> Self {
        Self(GridInfinite::new(1.0))
    }
}

#[derive(Default)]
pub struct Tile {}

impl<C> GridTile<C> for Tile
where
    C: GridCell,
{
    type Neighbors = C::Neighbors;

    fn is_walkable(&self) -> bool {
        false
    }

    fn neighbors(&self, cell: C) -> Self::Neighbors {
        cell.neighbors()
    }
}
