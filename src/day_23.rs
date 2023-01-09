use std::{
    collections::{HashMap, HashSet},
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

    let direction_order = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];

    let mut steps_completed = 0;

    for direction_start in 0..(steps as usize) {
        steps_completed += 1;
        let mut position_candidates: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        let mut changes = 0;
        for elf in &map {
            let mut candidate = elf.clone();

            // check surrounds
            let mut surronding_elves = 0;
            for x in elf.0 - 1..=elf.0 + 1 {
                for y in elf.1 - 1..=elf.1 + 1 {
                    if map.contains(&(x, y)) {
                        surronding_elves += 1;
                    }
                }
            }
            if surronding_elves > 1 {
                for direction_i in direction_start..direction_start + direction_order.len() {
                    let direction = direction_order[direction_i % direction_order.len()];

                    let sides = if direction.0 == 0 {
                        vec![(-1, 0), (1, 0)]
                    } else {
                        vec![(0, -1), (0, 1)]
                    };

                    let new_position = (elf.0 + direction.0, elf.1 + direction.1);
                    let mut check_positions = vec![new_position.clone()];
                    for s in sides {
                        check_positions.push((new_position.0 + s.0, new_position.1 + s.1));
                    }

                    let found = check_positions.iter().any(|p| map.contains(p));

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
