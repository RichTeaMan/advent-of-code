use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    io::{self},
};
use utils::coordinate::{Coordinate, char_coordinate_map_from_file};

pub fn day_8() -> io::Result<i32> {
    calc_antinodes("./inputs/day-8-input.txt")
}
pub fn day_8_part_2() -> io::Result<i32> {
    calc_resonant_antinodes("./inputs/day-8-input.txt")
}

fn calc_antinodes(filename: &str) -> io::Result<i32> {
    let (map, width, height) = char_coordinate_map_from_file(filename, true)?;

    let mut antennae_positions: HashMap<char, Vec<Coordinate>> = HashMap::new();
    for (k, v) in map {
        antennae_positions
            .entry(v)
            .and_modify(|e| e.push(k))
            .or_insert(vec![k]);
    }

    let mut antinodes = HashSet::new();
    for (_, positions) in antennae_positions {
        for combination in positions.iter().combinations(2) {
            debug_assert_eq!(combination.len(), 2);
            let a = combination[0];
            let b = combination[1];
            debug_assert_ne!(a, b);

            let diff = ((a.x - b.x), (a.y - b.y));

            let mut antinode_1 = a.clone();

            let mut antinode_2 = b.clone();
            antinode_1.add_tuple(diff);
            antinode_2.add_tuple((-diff.0, -diff.1));

            debug_assert_ne!(*a, antinode_1);
            debug_assert_ne!(*a, antinode_2);
            debug_assert_ne!(*b, antinode_1);
            debug_assert_ne!(*b, antinode_2);
            debug_assert_ne!(antinode_1, antinode_2);

            if antinode_1.in_bounds(width as i32, height as i32) {
                antinodes.insert(antinode_1);
            }
            if antinode_2.in_bounds(width as i32, height as i32) {
                antinodes.insert(antinode_2);
            }
        }
    }

    Ok(antinodes.len() as i32)
}

fn calc_resonant_antinodes(filename: &str) -> io::Result<i32> {
    let (map, width, height) = char_coordinate_map_from_file(filename, true)?;

    let mut antennae_positions: HashMap<char, Vec<Coordinate>> = HashMap::new();
    for (k, v) in map {
        antennae_positions
            .entry(v)
            .and_modify(|e| e.push(k))
            .or_insert(vec![k]);
    }

    let mut antinodes = HashSet::new();
    for (_, positions) in antennae_positions {
        for combination in positions.iter().combinations(2) {
            debug_assert_eq!(combination.len(), 2);
            let a = combination[0];
            let b = combination[1];
            debug_assert_ne!(a, b);

            antinodes.insert(a.clone());

            let diff = ((a.x - b.x), (a.y - b.y));

            let mut prev_node = a.clone();
            loop {
                let mut antinode = prev_node.clone();
                antinode.add_tuple(diff);

                if antinode.in_bounds(width as i32, height as i32) {
                    prev_node = antinode.clone();
                    antinodes.insert(antinode);
                } else {
                    break;
                }
            }

            let neg_diff = (-diff.0, -diff.1);
            let mut prev_node = a.clone();
            loop {
                let mut antinode = prev_node.clone();
                antinode.add_tuple(neg_diff);

                if antinode.in_bounds(width as i32, height as i32) {
                    prev_node = antinode.clone();
                    antinodes.insert(antinode);
                } else {
                    break;
                }
            }
        }
    }

    Ok(antinodes.len() as i32)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(calc_antinodes("./inputs/day-8-input-test.txt").unwrap(), 14);
    }

    #[test]
    fn test() {
        assert_eq!(calc_antinodes("./inputs/day-8-input.txt").unwrap(), 244);
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            calc_resonant_antinodes("./inputs/day-8-input-test.txt").unwrap(),
            34
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            calc_resonant_antinodes("./inputs/day-8-input.txt").unwrap(),
            912
        );
    }
}
