use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{self},
};

use crate::file_utils::read_lines;

const ENERGY_LIMIT: i32 = 10;

pub fn day_11() -> io::Result<i32> {
    run_steps("./inputs/day-11-input.txt", 100)
}
pub fn day_11_part_2() -> io::Result<i32> {
    find_first_synced_flash("./inputs/day-11-input.txt")
}

fn load_map(filename: &str) -> io::Result<HashMap<(i32, i32), i32>> {
    let mut map = HashMap::new();

    let lines = read_lines(filename)?;
    let mut y = 0;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }

        for (x, c) in line.trim().chars().enumerate() {
            let energy = format!("{c}").parse::<i32>().unwrap();
            map.insert((x as i32, y), energy);
        }

        y += 1;
    }
    Ok(map)
}

fn perform_step(mut map: HashMap<(i32, i32), i32>) -> (HashMap<(i32, i32), i32>, i32) {
    let mut to_flash = VecDeque::new();
    let mut flashed = HashSet::new();
    for ((x, y), energy) in map.iter_mut() {
        *energy += 1;
        if *energy >= ENERGY_LIMIT {
            to_flash.push_back((*x, *y));
        }
    }

    while let Some((x, y)) = to_flash.pop_front() {
        if flashed.contains(&(x, y)) {
            continue;
        }
        flashed.insert((x, y));

        let adjacent: Vec<(i32, i32)> = vec![
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ];

        for (adj_x, adj_y) in adjacent {
            if let Some(energy) = map.get_mut(&(adj_x, adj_y)) {
                *energy += 1;
                if *energy >= ENERGY_LIMIT {
                    to_flash.push_back((adj_x, adj_y));
                }
            }
        }
    }

    for (x, y) in flashed.iter() {
        if let Some(energy) = map.get_mut(&(*x, *y)) {
            *energy = 0;
        }
    }

    (map, flashed.len() as i32)
}

fn run_steps(filepath: &str, steps: usize) -> io::Result<i32> {
    let mut map = load_map(filepath)?;
    let mut flashes = 0;
    for _ in 0..steps {
        let (step_map, step_flashes) = perform_step(map);
        map = step_map;
        flashes += step_flashes;
    }
    Ok(flashes)
}

fn find_first_synced_flash(filepath: &str) -> io::Result<i32> {
    let mut map = load_map(filepath)?;
    for i in 0..1_000_000 {
        let (step_map, step_flashes) = perform_step(map);
        map = step_map;
        if step_flashes == 100 {
            return Ok(i + 1);
        }
    }
    panic!("Failed to converge.");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            run_steps("./inputs/day-11-input-test.txt", 100).unwrap(),
            1656
        );
    }

    #[test]
    fn test() {
        assert_eq!(run_steps("./inputs/day-11-input.txt", 100).unwrap(), 1608);
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            find_first_synced_flash("./inputs/day-11-input-test.txt").unwrap(),
            195
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            find_first_synced_flash("./inputs/day-11-input.txt").unwrap(),
            214
        );
    }
}
