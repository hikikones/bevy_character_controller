use bevy::utils::HashMap;

use crate::cell::*;

use super::{Grid, GridTile};

pub struct GridInfinite<C, T>
where
    C: GridCell,
    T: GridTile<C>,
{
    tiles: HashMap<C, T>,
    cell_size: CellFloat,
}

impl<C, T> GridInfinite<C, T>
where
    C: GridCell,
    T: GridTile<C>,
{
    pub fn new(cell_size: CellFloat) -> Self {
        Self {
            tiles: HashMap::new(),
            cell_size,
        }
    }
}

impl<C, T> Grid<C, T> for GridInfinite<C, T>
where
    C: GridCell,
    T: GridTile<C>,
{
    fn get_cell(&self, point: CellPointFloat) -> C {
        C::from_point(point, self.cell_size)
    }

    fn get_point(&self, cell: C) -> CellPointFloat {
        cell.as_point(self.cell_size)
    }

    fn get_tile(&self, cell: C) -> Option<&T> {
        self.tiles.get(&cell)
    }

    fn set_tile(&mut self, cell: C, tile: T) -> Option<T> {
        self.tiles.insert(cell, tile)
    }
}
