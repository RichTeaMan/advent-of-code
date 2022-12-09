use std::{
    collections::HashSet,
    fmt::Display, io,
};

use itertools::Itertools;

use crate::file_utils::read_lines;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
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
}
struct Rope {
    pub head: Point,
    pub tail: Point,
}

impl Rope {
    pub fn new() -> Rope {
        Rope {
            head: Point::origin(),
            tail: Point::origin(),
        }
    }

    pub fn move_up(mut self) -> Rope {
        let old = self.head.clone();
        self.head.y -= 1;
        if (self.head.y - self.tail.y).abs() > 1 {
            self.tail = old;
        }
        self
    }

    pub fn move_down(mut self) -> Rope {
        let old = self.head.clone();
        self.head.y += 1;
        if (self.head.y - self.tail.y).abs() > 1 {
            self.tail = old;
        }
        self
    }

    pub fn move_left(mut self) -> Rope {
        let old = self.head.clone();
        self.head.x -= 1;
        if (self.head.x - self.tail.x).abs() > 1 {
            self.tail = old;
        }
        self
    }

    pub fn move_right(mut self) -> Rope {
        let old = self.head.clone();
        self.head.x += 1;
        if (self.head.x - self.tail.x).abs() > 1 {
            self.tail = old;
        }
        self
    }
}

pub fn day_9() -> io::Result<i32> {
    let positions = fetch_tail_position_count("./inputs/day-9-input.txt")?;
    Ok(positions)
}

pub fn day_9_part_2() -> io::Result<usize> {
    todo!();
}

fn fetch_tail_position_count(filename: &str) -> io::Result<i32> {
    let mut positions = HashSet::new();
    let mut rope = Rope::new();
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
                    positions.insert(rope.tail.clone());
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
    fn diagonal_test() {
        let mut rope = Rope {
            head: Point { x: 5, y: 5 },
            tail: Point { x: 4, y: 4 },
        };
        rope = rope.move_right();
        assert_eq!(&rope.head, &Point { x: 6, y: 5 });
        assert_eq!(&rope.tail, &Point { x: 5, y: 5 });
    }

    #[test]
    fn small_test() {
        assert_eq!(
            fetch_tail_position_count("./inputs/day-9-input-test.txt").unwrap(),
            13
        );
    }

    #[test]
    fn test() {
        assert_eq!(fetch_tail_position_count("./inputs/day-9-input.txt").unwrap(), 6266)
    }
}
