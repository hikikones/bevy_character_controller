use crate::cell::*;

mod infinite;

pub use infinite::*;

pub trait Grid<C, T>
where
    C: GridCell,
    T: GridTile<C>,
{
    fn get_cell(&self, point: CellPointFloat) -> C;
    fn get_point(&self, cell: C) -> CellPointFloat;
    fn get_tile(&self, cell: C) -> Option<&T>;
    fn set_tile(&mut self, cell: C, tile: T) -> Option<T>;
}

pub trait GridTile<C>
where
    Self: Default,
    C: GridCell,
{
    type Neighbors: Iterator<Item = C>;

    fn is_walkable(&self) -> bool; // TODO: Use flags?
    fn neighbors(&self, cell: C) -> Self::Neighbors;

    fn heuristic(&self, cell: C, goal: C) -> CellUint {
        cell.distance(goal)
    }
}
