use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Add, Sub},
};

/**
 * Represents 3D coordinates. By convention, the origin (0, 0, 0) is the centre.
 */
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coordinate3d{
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Coordinate3d {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Coordinate3d { x, y, z }
    }

    pub fn origin() -> Self {
        Coordinate3d { x: 0, y: 0, z: 0 }
    }

    pub fn orthogonal(&self) -> Vec<Self> {
        vec![
            Coordinate3d {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            },
            Coordinate3d {
                x: self.x + 1,
                y: self.y,
                z:self.z,
            },
            Coordinate3d {
                x: self.x,
                y: self.y - 1,
                z:self.z,
            },
            Coordinate3d {
                x: self.x,
                y: self.y + 1,
                z:self.z,
            },
            Coordinate3d {
                x: self.x,
                y: self.y,
                z:self.z - 1,
            },
            Coordinate3d {
                x: self.x,
                y: self.y,
                z:self.z + 1,
            },
        ]
    }
}

impl Add for Coordinate3d {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate3d::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Coordinate3d {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Coordinate3d::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl From<(i32, i32, i32)> for Coordinate3d {
    fn from(item: (i32, i32, i32)) -> Self {
        Coordinate3d {
            x: item.0,
            y: item.1,
            z: item.2,
        }
    }
}

impl Display for Coordinate3d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("({x}, {y})", x = self.x, y = self.y).fmt(f)
    }
}

pub type Coordinate3dMap<T> = HashMap<Coordinate3d, T>;
