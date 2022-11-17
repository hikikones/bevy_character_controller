use std::ops::Add;

use bevy::math::{IVec2, IVec3, Quat, Vec3};

use crate::cell::*;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct SquareCell {
    column: CellInt,
    row: CellInt,
}

impl SquareCell {
    pub fn new(column: CellInt, row: CellInt) -> Self {
        Self { column, row }
    }
}

impl GridCell for SquareCell {
    type Neighbors = std::array::IntoIter<Self, 8>;
    type Direction = SquareDirection;
    type Directions = std::array::IntoIter<Self::Direction, 8>;

    fn column(&self) -> CellInt {
        self.column
    }

    fn row(&self) -> CellInt {
        self.row
    }

    fn floor(&self) -> CellInt {
        0
    }

    fn from_point(point: CellPointFloat, size: CellFloat) -> Self {
        Self {
            column: (point.x / size).floor() as CellInt,
            row: (point.z / size).floor() as CellInt,
        }
    }

    fn as_point(&self, size: CellFloat) -> CellPointFloat {
        let x = self.column as CellFloat * size;
        let z = self.row as CellFloat * size;
        let point = CellPointFloat::new(x, 0.0, z);
        let size_half_offset = CellPointFloat::new(size * 0.5, 0.0, size * 0.5);
        point + size_half_offset
    }

    fn neighbors(&self) -> Self::Neighbors {
        let cell = *self;
        [
            cell + Self::new(-1, 1),
            cell + Self::new(0, 1),
            cell + Self::new(1, 1),
            cell + Self::new(-1, 0),
            cell + Self::new(1, 0),
            cell + Self::new(-1, -1),
            cell + Self::new(0, -1),
            cell + Self::new(1, -1),
        ]
        .into_iter()
    }

    fn directions(&self) -> Self::Directions {
        [
            SquareDirection::North,
            SquareDirection::NorthEast,
            SquareDirection::East,
            SquareDirection::SouthEast,
            SquareDirection::South,
            SquareDirection::SouthWest,
            SquareDirection::West,
            SquareDirection::NorthWest,
        ]
        .into_iter()
    }
}

impl From<CellPointInt> for SquareCell {
    fn from(v: CellPointInt) -> Self {
        Self::new(v.x, v.z)
    }
}

impl Add<Self> for SquareCell {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.column + rhs.column, self.row + rhs.row)
    }
}

impl Add<IVec3> for SquareCell {
    type Output = Self;
    fn add(self, rhs: IVec3) -> Self::Output {
        Self::new(self.column + rhs.x, self.row + rhs.z)
    }
}

impl Sub<Self> for SquareCell {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.column - rhs.column, self.row - rhs.row)
    }
}

impl Sub<IVec3> for SquareCell {
    type Output = Self;
    fn sub(self, rhs: IVec3) -> Self::Output {
        Self::new(self.column - rhs.x, self.row - rhs.z)
    }
}

impl Mul<CellInt> for SquareCell {
    type Output = Self;
    fn mul(self, rhs: CellInt) -> Self::Output {
        Self::new(self.column * rhs, self.row * rhs)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SquareDirection {
    North = 0,
    NorthEast = 1,
    East = 2,
    SouthEast = 3,
    South = 4,
    SouthWest = 5,
    West = 6,
    NorthWest = 7,
}

impl Default for SquareDirection {
    fn default() -> Self {
        Self::North
    }
}

impl Into<IVec2> for SquareDirection {
    fn into(self) -> IVec2 {
        match self {
            SquareDirection::North => -IVec2::new(0, -1),
            SquareDirection::NorthEast => IVec2::new(1, -1),
            SquareDirection::East => IVec2::new(1, 0),
            SquareDirection::SouthEast => IVec2::new(1, 1),
            SquareDirection::South => IVec2::new(0, 1),
            SquareDirection::SouthWest => IVec2::new(-1, 1),
            SquareDirection::West => IVec2::new(-1, 0),
            SquareDirection::NorthWest => IVec2::new(-1, -1),
        }
    }
}

impl Into<IVec3> for SquareDirection {
    fn into(self) -> IVec3 {
        let v: IVec2 = self.into();
        IVec3::new(v.x, 0, v.y)
    }
}

impl Into<Vec3> for SquareDirection {
    fn into(self) -> Vec3 {
        let v: IVec2 = self.into();
        Vec3::new(v.x as f32, 0.0, v.y as f32)
    }
}

impl Into<Quat> for SquareDirection {
    fn into(self) -> Quat {
        let i = self as u8;
        let r = i as f32 * std::f32::consts::FRAC_PI_4;
        Quat::from_rotation_y(-r)
    }
}

impl From<u8> for SquareDirection {
    fn from(i: u8) -> Self {
        let index = i % 8;
        match index {
            0 => SquareDirection::North,
            1 => SquareDirection::NorthEast,
            2 => SquareDirection::East,
            3 => SquareDirection::SouthEast,
            4 => SquareDirection::South,
            5 => SquareDirection::SouthWest,
            6 => SquareDirection::West,
            7 => SquareDirection::NorthWest,
            _ => panic!("Could not convert {index} into SquareDirection"),
        }
    }
}

impl CellDirection for SquareDirection {
    fn rotate_clockwise(self) -> Self {
        let next = self as u8 + 1;
        next.into()
    }

    fn rotate_counter_clockwise(self) -> Self {
        let index = self as u8;

        if index == 0 {
            return SquareDirection::NorthWest;
        }

        let previous = index - 1;
        previous.into()
    }
}
