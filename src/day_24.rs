use std::{
    collections::{HashMap, HashSet},
    io,
};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::file_utils::read_lines;

enum Destination {
    Beginning,
    End,
}

#[derive(Clone, Debug, EnumIter)]
enum Direction {
    North,
    East,
    South,
    West,
    None,
}

impl Direction {
    fn calculate_position(&self, x: i32, y: i32) -> (i32, i32) {
        let (dx, dy) = match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::None => (0, 0),
        };
        (x + dx, y + dy)
    }
}

#[derive(Clone, Debug)]
struct Blizzard {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Blizzard {
    pub fn process(&mut self) {
        let (x, y) = self.direction.calculate_position(self.x, self.y);
        self.x = x;
        self.y = y;
    }
}

#[derive(Clone, Debug)]
struct Map {
    width: i32,
    height: i32,
    minute: i32,
    blizzards: Vec<Blizzard>,

    occupied_space: HashSet<(i32, i32)>,
}

impl Map {
    fn build_occupied_space(&mut self) {
        self.occupied_space.clear();
        for blizzard in &self.blizzards {
            self.occupied_space.insert((blizzard.x, blizzard.y));
        }
    }

    fn tick(&mut self) {
        for blizzard in self.blizzards.iter_mut() {
            blizzard.process();

            if blizzard.x < 0 {
                blizzard.x = self.width - 1;
            } else if blizzard.x >= self.width {
                blizzard.x = 0;
            }

            if blizzard.y < 0 {
                blizzard.y = self.height - 1;
            } else if blizzard.y >= self.height {
                blizzard.y = 0;
            }
        }
        self.build_occupied_space();
        self.minute += 1;
    }

    fn is_free(&self, x: i32, y: i32) -> bool {
        x >= 0
            && x < self.width
            && y >= 0
            && y < self.height
            && !self.occupied_space.contains(&(x, y))
    }

    #[allow(dead_code)]
    fn print(&self) {
        let mut taken: HashMap<(i32, i32), (i32, Direction)> = HashMap::new();
        for blizzard in &self.blizzards {
            let coords = (blizzard.x, blizzard.y);
            let mut v = 1;
            if taken.contains_key(&coords) {
                let (o_v, _) = taken[&coords];
                v = o_v + 1;
            }

            taken.insert(coords, (v, blizzard.direction.clone()));
        }

        println!("Minute {t}:", t = self.minute);
        for y in 0..self.height {
            for x in 0..self.width {
                let mut glyph = ".".to_string();
                if let Some((count, direction)) = taken.get(&(x, y)) {
                    if *count == 1 {
                        let g = match direction {
                            Direction::North => "^",
                            Direction::East => ">",
                            Direction::South => "v",
                            Direction::West => "<",
                            Direction::None => ".",
                        };
                        glyph = g.to_string();
                    } else {
                        debug_assert!(*count < 10);
                        glyph = count.to_string();
                    }
                }
                print!("{glyph}");
            }
            println!();
        }
        println!();
    }
}

fn load_map(filename: &str) -> io::Result<Map> {
    let mut width_opt = None;
    let mut height_opt = None;

    let mut blizzards = Vec::new();

    let lines = read_lines(filename)?;
    for (y, line) in lines.flatten().enumerate() {
        if line.is_empty() {
            continue;
        }

        // start or end
        if line.chars().into_iter().filter(|c| *c == '#').count() > 2 {
            if width_opt.is_none() {
                width_opt = Some(line.chars().count() as i32 - 2);
            } else {
                height_opt = Some(y as i32 - 1);
            }
            continue;
        }

        for (x, location) in line.chars().enumerate() {
            let direction_opt = match location {
                '#' => None,
                '.' => None,
                '>' => Some(Direction::East),
                '<' => Some(Direction::West),
                '^' => Some(Direction::North),
                'v' => Some(Direction::South),
                c => panic!("Unknown input: {c}"),
            };

            if let Some(direction) = direction_opt {
                blizzards.push(Blizzard {
                    x: x as i32 - 1,
                    y: y as i32 - 1,
                    direction,
                });
            }
        }
    }

    debug_assert!(width_opt.is_some());
    debug_assert!(height_opt.is_some());
    debug_assert!(width_opt.unwrap() > 0);
    debug_assert!(height_opt.unwrap() > 0);

    let mut map = Map {
        width: width_opt.unwrap(),
        height: height_opt.unwrap(),
        minute: 0,
        blizzards,
        occupied_space: HashSet::new(),
    };
    map.build_occupied_space();
    Ok(map)
}

fn calculate(destination: Destination, mut map: Map) -> Map {

    let start = match destination {
        Destination::Beginning => (map.width - 1, map.height - 1),
        Destination::End => (0, 0),
    };

    let target = match destination {
        Destination::Beginning => (0, 0),
        Destination::End => (map.width - 1, map.height - 1),
    };

    // player can only move down at first. wait until that option is available
    while !map.is_free(start.0, start.1) {
        map.tick();
    }

    let mut candidates: HashSet<(i32, i32)> = HashSet::from_iter(vec![start]);

    let mut destination_reached = false;

    while !destination_reached {

        map.tick();

        if map.minute > 10_000 {
            panic!("Fail, too many steps.");
        }

        let mut new_candidates = HashSet::new();
        // there is always an option to start again.
        new_candidates.insert(start.clone());

        for (candidate_x, candidate_y) in &candidates {
            for direction in Direction::iter() {
                let (x, y) = direction.calculate_position(*candidate_x, *candidate_y);

                if map.is_free(x, y) {
                    if (x, y) == target {
                        map.tick();
                        destination_reached = true;
                    }
                    new_candidates.insert((x, y));
                }
            }
        }
        candidates = new_candidates;
    }

    map
}

pub fn day_24() -> io::Result<i32> {
    let map = load_map("./inputs/day-24-input.txt")?;
    let result = calculate(Destination::End, map);
    Ok(result.minute)
}

pub fn day_24_part_2() -> io::Result<i32> {
    let map = load_map("./inputs/day-24-input.txt")?;
    let leg_1 = calculate(Destination::End, map);
    let leg_2 = calculate(Destination::Beginning, leg_1);
    let leg_3 = calculate(Destination::End, leg_2);
    Ok(leg_3.minute)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        let map = load_map("./inputs/day-24-input-test.txt").unwrap();
        let result = calculate(Destination::End, map);
        assert_eq!(18, result.minute);
    }

    #[test]
    fn test() {
        let map = load_map("./inputs/day-24-input.txt").unwrap();
        let result = calculate(Destination::End, map);
        assert_eq!(281, result.minute);
    }

    #[test]
    fn part_2_small_test() {
        let map = load_map("./inputs/day-24-input-test.txt").unwrap();

        println!("start");

        let leg_1 = calculate(Destination::End, map);
        assert_eq!(18, leg_1.minute);

        println!("leg 1 done");

        let leg_2 = calculate(Destination::Beginning, leg_1);
        assert_eq!(41, leg_2.minute);

        println!("leg 2 done");

        let leg_3 = calculate(Destination::End, leg_2);
        assert_eq!(54, leg_3.minute);
    }

    #[test]
    fn part_2_test() {
        let map = load_map("./inputs/day-24-input.txt").unwrap();
        let leg_1 = calculate(Destination::End, map);
        let leg_2 = calculate(Destination::Beginning, leg_1);
        let leg_3 = calculate(Destination::End, leg_2);
        assert_eq!(807, leg_3.minute);
    }
}
