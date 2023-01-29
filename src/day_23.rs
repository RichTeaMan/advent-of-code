use std::{
    collections::{HashMap, HashSet, VecDeque},
    io,
};

use crate::file_utils::read_lines;

type Map = HashSet<(i32, i32)>;

fn load_map(filename: &str) -> io::Result<Map> {
    let mut map = Map::new();

    let lines = read_lines(filename)?;
    for (y, line) in lines.flatten().enumerate() {
        if line.is_empty() {
            continue;
        }

        for (x, location) in line.chars().enumerate() {
            if location == '#' {
                map.insert((x as i32, y as i32));
            }
        }
    }

    Ok(map)
}

fn calculate_elves(steps: i32, filename: &str) -> io::Result<(i32, i32)> {
    let mut map = load_map(filename)?;

    let mut direction_order = VecDeque::from(vec![(0, -1), (0, 1), (-1, 0), (1, 0)]);

    let mut steps_completed = 0;

    for _ in 0..(steps as usize) {
        steps_completed += 1;
        let mut position_candidates: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        let mut changes = 0;
        for elf in &map {
            let mut candidate = *elf;

            // check surrounds
            let mut surrounding_elves =
                map.contains(&(elf.0 - 1, elf.1)) || map.contains(&(elf.0 + 1, elf.1));
            if !surrounding_elves {
                for x in elf.0 - 1..=elf.0 + 1 {
                    if map.contains(&(x, elf.1 - 1)) || map.contains(&(x, elf.1 + 1)) {
                        surrounding_elves = true;
                        break;
                    }
                }
            }

            if surrounding_elves {
                for direction in &direction_order {
                    let new_position = (elf.0 + direction.0, elf.1 + direction.1);
                    let side_1;
                    let side_2;

                    if direction.0 == 0 {
                        side_1 = (new_position.0 - 1, new_position.1);
                        side_2 = (new_position.0 + 1, new_position.1);
                    } else {
                        side_1 = (new_position.0, new_position.1 - 1);
                        side_2 = (new_position.0, new_position.1 + 1);
                    };

                    let found = map.contains(&new_position)
                        || map.contains(&side_1)
                        || map.contains(&side_2);

                    if !found {
                        changes += 1;
                        candidate = new_position;
                        break;
                    }
                }
            }

            if let Some(existing) = position_candidates.remove(&candidate) {
                debug_assert_ne!(existing, candidate);

                position_candidates.insert(existing, existing);
                position_candidates.insert(*elf, *elf);
                changes -= 1;
            } else {
                position_candidates.insert(candidate, *elf);
            }
        }

        if changes == 0 {
            break;
        }

        debug_assert!(changes >= 0);

        map.clear();
        for (position_candidate, _) in position_candidates {
            map.insert(position_candidate);
        }

        let f = direction_order.pop_front().unwrap();
        direction_order.push_back(f);
    }

    // find smallest rectangle
    let min_x = map.iter().map(|(x, _)| x).min().unwrap().to_owned();
    let max_x = map.iter().map(|(x, _)| x).max().unwrap().to_owned() + 1;

    let min_y = map.iter().map(|(_, y)| y).min().unwrap().to_owned();
    let max_y = map.iter().map(|(_, y)| y).max().unwrap().to_owned() + 1;

    let width = max_x - min_x;
    let height = max_y - min_y;
    let empty = width * height - map.len() as i32;

    Ok((empty, steps_completed))
}

pub fn day_23() -> io::Result<i32> {
    let (result, _) = calculate_elves(10, "./inputs/day-23-input.txt")?;
    Ok(result)
}

pub fn day_23_part_2() -> io::Result<i32> {
    let (_, steps) = calculate_elves(100_000, "./inputs/day-23-input.txt")?;
    Ok(steps)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        let (result, _) = calculate_elves(10, "./inputs/day-23-input-test.txt").unwrap();
        assert_eq!(110, result);
    }

    #[test]
    fn test() {
        let (result, _) = calculate_elves(10, "./inputs/day-23-input.txt").unwrap();
        assert_eq!(4236, result);
    }

    #[test]
    fn part_2_small_test() {
        let (_, steps) = calculate_elves(100, "./inputs/day-23-input-test.txt").unwrap();
        assert_eq!(20, steps);
    }

    #[test]
    fn part_2_test() {
        let (_, steps) = calculate_elves(100_000, "./inputs/day-23-input.txt").unwrap();
        assert_eq!(1023, steps);
    }
}
