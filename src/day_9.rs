use std::{collections::HashSet, fmt::Display, io};

use itertools::Itertools;

use crate::file_utils::read_lines;

#[derive(PartialEq, Eq, Clone, Hash, Debug, Copy, Ord, PartialOrd)]
struct Point {
    pub x: i32,
    pub y: i32,
}
impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({x}, {y})", x = self.x, y = self.y)
    }
}
impl Point {
    pub fn origin() -> Point {
        Point { x: 0, y: 0 }
    }

    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn abs(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    pub fn fetch_travel_vector(a: Point, b: Point) -> Point {
        let diff = Point::new(a.x - b.x, a.y - b.y);

        if diff.x.abs() < 2 && diff.y.abs() < 2 {
            return Point::new(0, 0);
        }
        if diff.x == 0 {
            return Point::new(0, diff.y / 2);
        }
        if diff.y == 0 {
            return Point::new(diff.x / 2, 0);
        }

        // diagonal
        Point::new(
            if diff.x.is_negative() { -1 } else { 1 },
            if diff.y.is_negative() { -1 } else { 1 },
        )
    }
}

#[derive(Clone)]
struct Rope {
    pub knots: Vec<Point>,
}

impl Rope {
    pub fn new(length: i32) -> Rope {
        let mut knots = Vec::new();
        for _ in 0..length {
            knots.push(Point::origin());
        }
        Rope { knots }
    }

    pub fn move_up(self) -> Rope {
        self.move_by(Point::new(0, -1))
    }

    pub fn move_down(self) -> Rope {
        self.move_by(Point::new(0, 1))
    }

    pub fn move_left(self) -> Rope {
        self.move_by(Point::new(-1, 0))
    }

    pub fn move_right(self) -> Rope {
        self.move_by(Point::new(1, 0))
    }

    pub fn move_by(mut self, point: Point) -> Rope {
        if point.x == 0 && point.y == 0 {
            return self;
        }

        self.knots[0].x += point.x;
        self.knots[0].y += point.y;

        let mut distance;
        for i in 1..self.knots.len() {
            let a = self.knots[i - 1];
            let b = self.knots[i];
            distance = Point::fetch_travel_vector(a, b);
            if distance.abs() > 2 {
                panic!("big distance: {a:?} - {b:?} = {distance:?}");
            }
            self.knots[i].x += distance.x;
            self.knots[i].y += distance.y;
        }
        self
    }
}

pub fn day_9() -> io::Result<i32> {
    let positions = fetch_tail_position_count("./inputs/day-9-input.txt")?;
    Ok(positions)
}

pub fn day_9_part_2() -> io::Result<i32> {
    let positions = fetch_long_tail_position_count(10, "./inputs/day-9-input.txt")?;
    Ok(positions)
}

fn fetch_tail_position_count(filename: &str) -> io::Result<i32> {
    let mut positions = HashSet::new();
    let mut rope = Rope::new(2);
    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }
        if let Some((direction, number_s)) = line.split(' ').collect_tuple() {
            if let Ok(number) = number_s.parse::<i32>() {
                for _ in 0..number {
                    rope = match direction {
                        "U" => rope.move_up(),
                        "L" => rope.move_left(),
                        "D" => rope.move_down(),
                        "R" => rope.move_right(),
                        d => panic!("Unknown movement type '{d}'."),
                    };
                    positions.insert(*rope.knots.last().unwrap());
                }
            } else {
                panic!("Bad number input");
            }
        }
    }
    Ok(positions.len() as i32)
}

fn fetch_long_tail_position_count(length: i32, filename: &str) -> io::Result<i32> {
    let mut positions = HashSet::new();
    let mut rope = Rope::new(length);
    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }
        if let Some((direction, number_s)) = line.split(' ').collect_tuple() {
            if let Ok(number) = number_s.parse::<i32>() {
                for _ in 0..number {
                    rope = match direction {
                        "U" => rope.move_up(),
                        "L" => rope.move_left(),
                        "D" => rope.move_down(),
                        "R" => rope.move_right(),
                        d => panic!("Unknown movement type '{d}'."),
                    };
                    positions.insert(*rope.knots.last().unwrap());
                }
            } else {
                panic!("Bad number input");
            }
        }
    }

    Ok(positions.len() as i32)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            fetch_tail_position_count("./inputs/day-9-input-test.txt").unwrap(),
            13
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            fetch_tail_position_count("./inputs/day-9-input.txt").unwrap(),
            6266
        )
    }

    #[test]
    fn very_small_part_2_test() {
        assert_eq!(
            fetch_long_tail_position_count(10, "./inputs/day-9-input-test.txt").unwrap(),
            1
        );
    }

    #[test]
    fn small_part_2_test() {
        assert_eq!(
            fetch_long_tail_position_count(10, "./inputs/day-9-input-part-2-test.txt").unwrap(),
            36
        );
    }
}
