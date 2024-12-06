use std::{
    collections::HashMap,
    fmt::Display,
    io,
    ops::{Add, Sub},
};

use crate::file_utils::read_lines;

/**
 * Represents cartesian coordinates. By convention, the origin (0, 0) is the top-left corner.
 */
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

pub static COORDINATE_VECTORS: &'static [(i32, i32)] = &[
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Coordinate { x, y }
    }

    pub fn origin() -> Self {
        Coordinate { x: 0, y: 0 }
    }

    pub fn add_tuple(&mut self, other: (i32, i32)) {
        self.x += other.0;
        self.y += other.1;
    }

    pub fn orthogonal(&self) -> Vec<Self> {
        vec![
            Coordinate {
                x: self.x - 1,
                y: self.y,
            },
            Coordinate {
                x: self.x + 1,
                y: self.y,
            },
            Coordinate {
                x: self.x,
                y: self.y - 1,
            },
            Coordinate {
                x: self.x,
                y: self.y + 1,
            },
        ]
    }

    /**
     * Gets all surrounding coordinates, including diagonals.
     */
    pub fn surround(&self) -> Vec<Self> {
        vec![
            Coordinate {
                x: self.x - 1,
                y: self.y,
            },
            Coordinate {
                x: self.x + 1,
                y: self.y,
            },
            Coordinate {
                x: self.x,
                y: self.y - 1,
            },
            Coordinate {
                x: self.x,
                y: self.y + 1,
            },
            Coordinate {
                x: self.x - 1,
                y: self.y - 1,
            },
            Coordinate {
                x: self.x + 1,
                y: self.y - 1,
            },
            Coordinate {
                x: self.x - 1,
                y: self.y + 1,
            },
            Coordinate {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Coordinate::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl From<(i32, i32)> for Coordinate {
    fn from(item: (i32, i32)) -> Self {
        Coordinate {
            x: item.0,
            y: item.1,
        }
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("({x}, {y})", x = self.x, y = self.y).fmt(f)
    }
}

pub type CoordinateMap<T> = HashMap<Coordinate, T>;

/**
 * (0, 0) is top left coordinate.
 */
pub fn char_coordinate_map_from_file(
    filename: &str,
    ignore_dot: bool,
) -> io::Result<CoordinateMap<char>> {
    let lines = read_lines(filename)?;
    let mut coordinate_map = CoordinateMap::new();

    for (y, line) in lines.flatten().enumerate() {
        if line.is_empty() {
            continue;
        }

        for (x, c) in line.chars().enumerate() {
            if ignore_dot && c == '.' {
                continue;
            }
            coordinate_map.insert(Coordinate::new(x as i32, y as i32), c);
        }
    }
    Ok(coordinate_map)
}
