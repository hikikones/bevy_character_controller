use bevy::prelude::IVec3;

use crate::{CellInt, CellUint, GridCell};

pub struct CellBoxIter<C>
where
    C: GridCell,
{
    start: C,
    cols: CellUint,
    length: CellUint,
    counter: CellUint,
}

impl<C> CellBoxIter<C>
where
    C: GridCell,
{
    pub fn new(start: C, cols: CellUint, rows: CellUint) -> Self {
        Self {
            start,
            cols,
            length: cols * rows,
            counter: 0,
        }
    }
}

impl<C> Iterator for CellBoxIter<C>
where
    C: GridCell,
{
    type Item = C;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter == self.length {
            return None;
        }

        let col = self.counter % self.cols;
        let row = self.counter / self.cols;
        let cell = self.start + IVec3::new(col as CellInt, 0, row as CellInt);

        self.counter += 1;

        Some(cell)
    }
}

pub struct CellDirectionIter<C>
where
    C: GridCell,
{
    start: C,
    loops: CellInt,
    round: CellInt,
    dirs: C::Directions,
}

impl<C> CellDirectionIter<C>
where
    C: GridCell,
{
    pub fn new(start: C, loops: CellInt) -> Self {
        Self {
            start,
            loops,
            round: 1,
            dirs: start.directions(),
        }
    }
}

impl<C> Iterator for CellDirectionIter<C>
where
    C: GridCell,
{
    type Item = C;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(dir) = self.dirs.next() {
            return Some(self.start + Into::<IVec3>::into(dir) * self.round);
        }

        self.round += 1;

        if self.round > self.loops {
            return None;
        }

        self.dirs = self.start.directions();

        self.next()
    }
}
