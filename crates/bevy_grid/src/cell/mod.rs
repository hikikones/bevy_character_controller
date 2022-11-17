use std::hash::Hash;
use std::ops::Add;

use bevy::math::{IVec3, Quat, Vec3};

mod square_cell;

pub use square_cell::*;

pub type CellInt = i32;
pub type CellUint = u32;
pub type CellFloat = f32;
pub type CellPointFloat = Vec3;
pub type CellPointInt = IVec3;

pub trait GridCell
where
    Self: Default
        + Clone
        + Copy
        + PartialEq
        + Eq
        + Hash
        + From<CellPointInt>
        + Add<Self, Output = Self>
        + Add<CellPointInt, Output = Self>,
{
    type Neighbors: Iterator<Item = Self>;
    type Direction: CellDirection;

    fn column(&self) -> CellInt;
    fn row(&self) -> CellInt;
    fn floor(&self) -> CellInt;

    fn from_point(point: CellPointFloat, size: CellFloat) -> Self;
    fn as_point(&self, size: CellFloat) -> CellPointFloat;

    fn neighbors(&self) -> Self::Neighbors;

    // Default impls
    fn adjacent(&self, direction: Self::Direction) -> Self {
        let dir: IVec3 = direction.into();
        *self + dir
    }

    fn distance(&self, other: Self) -> CellUint {
        let x = other.column() - self.column();
        let y = other.row() - self.row();
        let z = other.floor() - self.floor();
        (x * x + y * y + z * z) as CellUint
    }
}

pub trait CellDirection
where
    Self: Default + Clone + Copy + Into<CellPointInt> + Into<Quat>,
{
    fn rotate_clockwise(self) -> Self;
    fn rotate_counter_clockwise(self) -> Self;
}
