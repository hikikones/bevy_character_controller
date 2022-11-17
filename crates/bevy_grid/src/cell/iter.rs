use crate::{CellFloat, CellInt, CellUint, GridCell};

pub struct CellDirectionIter<C>
where
    C: GridCell,
{
    start: C,
    loops: CellInt,
    round: CellInt,
    index: usize,
    dirs: Box<[C]>,
}

impl<C> CellDirectionIter<C>
where
    C: GridCell,
{
    pub fn new(start: C, loops: CellInt) -> Self {
        let dirs = start
            .directions()
            .map(|dir| {
                let neighbor = start.adjacent(dir);
                neighbor - start
            })
            .collect();

        Self {
            start,
            loops,
            round: 1,
            index: 0,
            dirs,
        }
    }
}

impl<C> Iterator for CellDirectionIter<C>
where
    C: GridCell,
{
    type Item = C;

    fn next(&mut self) -> Option<Self::Item> {
        if self.round > self.loops {
            return None;
        }

        let next = self.start + self.dirs[self.index] * self.round;

        self.index += 1;

        if self.index == self.dirs.len() {
            self.round += 1;
            self.index = 0;
        }

        Some(next)
    }
}
