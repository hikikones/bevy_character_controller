use bevy::prelude::IVec3;

use crate::{CellInt, GridCell};

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
