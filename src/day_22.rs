use std::{collections::HashMap, io};

use crate::file_utils::read_lines;

type Map = HashMap<i32, HashMap<i32, MapSection>>;

pub fn day_22() -> io::Result<i32> {
    let result = map_puzzle("./inputs/day-22-input.txt")?;
    Ok(result)
}

pub fn day_22_part_2() -> io::Result<i32> {
    todo!();
}

enum MapSection {
    WALL,
    FLOOR,
}

enum Direction {
    LEFT,
    RIGHT,
    NONE
}

#[derive(Debug)]

enum Facing {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl Facing {
    fn rotate_left(&self) -> Facing {
        match self {
            Facing::NORTH => Facing::WEST,
            Facing::EAST => Facing::NORTH,
            Facing::SOUTH => Facing::EAST,
            Facing::WEST => Facing::SOUTH,
        }
    }

    fn rotate_right(&self) -> Facing {
        match self {
            Facing::NORTH => Facing::EAST,
            Facing::EAST => Facing::SOUTH,
            Facing::SOUTH => Facing::WEST,
            Facing::WEST => Facing::NORTH,
        }
    }

    fn fetch_digit(&self) -> i32 {
        match self {
            Facing::NORTH => 3,
            Facing::EAST => 0,
            Facing::SOUTH => 1,
            Facing::WEST => 2,
        }
    }
}

struct Instruction {
    pub steps: i32,
    pub direction: Direction,
}

fn map_puzzle(filename: &str) -> io::Result<i32> {
    let mut map: Map = HashMap::new();
    let mut instructions = Vec::new();
    let lines = read_lines(filename)?;
    for (y, line) in lines.flatten().enumerate() {
        if line.is_empty() {
            continue;
        }

        if line.contains("L") || line.contains("R") {
            let directions = line.split_inclusive(&['L', 'R']);

            for direction in directions {
                let steps_str = direction.trim_end_matches(&['L', 'R']);
                let steps = steps_str.parse::<i32>().unwrap();
                instructions.push(Instruction {
                    steps,
                    direction: if direction.contains('L') {
                        Direction::LEFT
                    } else if direction.contains('R') {
                        Direction::RIGHT
                    } else {
                        Direction::NONE
                    },
                });
            }
            continue;
        }

        let mut map_line = HashMap::new();
        for (x, c) in line.chars().enumerate() {
            let section_opt = match c {
                ' ' => None,
                '.' => Some(MapSection::FLOOR),
                '#' => Some(MapSection::WALL),
                other => panic!("Unknown input {other}"),
            };
            if let Some(section) = section_opt {
                map_line.insert(x as i32, section);
            }
        }
        map.insert(y as i32, map_line);
    }

    // start is left most top tile

    let mut x = map
        .get(&0)
        .unwrap()
        .keys()
        .into_iter()
        .min()
        .unwrap()
        .to_owned();
    let mut y = 0;
    let mut facing = Facing::EAST;

    for instruction in instructions {
        for _ in 0..instruction.steps {
            let delta = match facing {
                Facing::NORTH => (0, -1),
                Facing::EAST => (1, 0),
                Facing::SOUTH => (0, 1),
                Facing::WEST => (-1, 0),
            };
            let mut new_x = x + delta.0;
            let mut new_y = y + delta.1;

            // do a wrap around
            if fetch_tile(&map, new_x, new_y).is_none() {
                if delta.0 < 0 {
                    new_x = max_x(&map, new_y);
                } else if delta.0 > 0 {
                    new_x = min_x(&map, new_y);
                }
                if delta.1 < 0 {
                    new_y = max_y(&map, new_x);
                } else if delta.1 > 0 {
                    new_y = min_y(&map, new_x);
                }
            }

            if let Some(tile) = fetch_tile(&map, new_x, new_y) {
                match tile {
                    MapSection::WALL => {
                        break;
                    }
                    MapSection::FLOOR => {
                        x = new_x;
                        y = new_y;
                    }
                }
            } else {
                panic!("Could not fetch tile with wrapped coords {new_x}, {new_y}")
            }
        }

        facing = match instruction.direction {
            Direction::LEFT => facing.rotate_left(),
            Direction::RIGHT => facing.rotate_right(),
            Direction::NONE => facing,
        };
    }


    // The final password is the sum of 1000 times the row, 4 times the column, and the facing.
    Ok(1000 * (y + 1) + 4 * (x + 1) + facing.fetch_digit())
}

fn fetch_tile(map: &Map, x: i32, y: i32) -> Option<&MapSection> {
    if let Some(line) = map.get(&y) {
        return line.get(&x);
    }
    None
}

fn min_x(map: &Map, y: i32) -> i32 {
    map.get(&y)
        .unwrap()
        .keys()
        .into_iter()
        .min()
        .unwrap()
        .to_owned()
}

fn max_x(map: &Map, y: i32) -> i32 {
    map.get(&y)
        .unwrap()
        .keys()
        .into_iter()
        .max()
        .unwrap()
        .to_owned()
}

fn min_y(map: &Map, x: i32) -> i32 {
    let mut ys = Vec::new();
    for (y, line) in map {
        if line.get(&x).is_some() {
            ys.push(y);
        }
    }
    *ys.iter().min().unwrap().to_owned()
}

fn max_y(map: &Map, x: i32) -> i32 {
    let mut ys = Vec::new();
    for (y, line) in map {
        if line.get(&x).is_some() {
            ys.push(y);
        }
    }
    *ys.iter().max().unwrap().to_owned()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            map_puzzle("./inputs/day-22-input-test.txt").unwrap(),
            6032
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            map_puzzle("./inputs/day-22-input.txt").unwrap(),
            103224
        );
    }
}
