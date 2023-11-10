use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    io::{self},
};

use crate::file_utils::read_lines;

#[derive(Debug, PartialEq, Eq)]
struct DjNode {
    coord: (i32, i32),
    total_cost: i32,
}

impl Default for DjNode {
    fn default() -> Self {
        Self {
            coord: (0, 0),
            total_cost: i32::MAX,
        }
    }
}

impl Ord for DjNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.total_cost.cmp(&self.total_cost)
    }
}

impl PartialOrd for DjNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn day_15() -> io::Result<i32> {
    find_low_risk("./inputs/day-15-input.txt")
}
pub fn day_15_part_2() -> io::Result<i32> {
    find_big_map_low_risk("./inputs/day-15-input.txt")
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

fn load_big_map(filename: &str) -> io::Result<HashMap<(i32, i32), i32>> {
    let mut tile = HashMap::new();

    let lines = read_lines(filename)?;
    let mut y = 0;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }

        for (x, c) in line.trim().chars().enumerate() {
            let risk = format!("{c}").parse::<i32>().unwrap();
            tile.insert((x as i32, y), risk);
        }

        y += 1;
    }

    let mut map = HashMap::new();

    let width = tile.keys().map(|(x, _)| x).max().unwrap().to_owned() + 1;
    let height = tile.keys().map(|(_, y)| y).max().unwrap().to_owned() + 1;

    debug_assert_eq!(0, width % 10);
    debug_assert_eq!(0, height % 10);

    for i in 0..5 {
        for j in 0..5 {
            let risk_increase = i + j;

            for ((tile_x, tile_y), risk) in &tile {
                let x = (i * width) + tile_x;
                let y = (j * height) + tile_y;
                debug_assert!(!map.contains_key(&(x, y)));

                // risk over 9 wraps around to 1. this is not how maths works so do some weird stuff instead.
                let new_risk = (((risk - 1) + risk_increase) % 9) + 1;
                debug_assert!(new_risk > 0 && new_risk <= 9);
                debug_assert!(
                    i > 0 || j > 0 || new_risk == *risk,
                    "i: {}, j: {}, risk: {}, new_risk: {}",
                    i,
                    j,
                    risk,
                    new_risk
                );
                map.insert((x, y), new_risk);
            }
        }
    }

    Ok(map)
}

fn find_low_risk(filepath: &str) -> io::Result<i32> {
    let map = load_map(filepath)?;
    Ok(find_low_risk_of_map(map))
}

fn find_big_map_low_risk(filepath: &str) -> io::Result<i32> {
    let map = load_big_map(filepath)?;
    Ok(find_low_risk_of_map(map))
}

fn find_low_risk_of_map(map: HashMap<(i32, i32), i32>) -> i32 {
    let end_x = map.keys().map(|(x, _)| x).max().unwrap().to_owned();
    let end_y = map.keys().map(|(_, y)| y).max().unwrap().to_owned();
    let end = (end_x, end_y);

    let mut unvisited = HashSet::new();
    for c in map.keys() {
        unvisited.insert(*c);
    }

    let mut heap: BinaryHeap<DjNode> = BinaryHeap::new();
    heap.push(DjNode {
        coord: (0, 0),
        total_cost: 0,
    });

    while let Some(current) = heap.pop() {
        let (x, y) = current.coord;
        let current_risk = current.total_cost;

        if current.coord == end {
            return current_risk;
        }

        if !unvisited.contains(&current.coord) {
            continue;
        }

        debug_assert!(current_risk != i32::MAX);
        let adjacent: Vec<(i32, i32)> = vec![(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)];

        for coord in adjacent {
            if unvisited.contains(&coord) {
                let new_risk = current_risk + map.get(&coord).unwrap();
                heap.push(DjNode {
                    coord,
                    total_cost: new_risk,
                });
            }
        }
        unvisited.remove(&current.coord);
    }

    unreachable!()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(find_low_risk("./inputs/day-15-input-test.txt").unwrap(), 40);
    }

    #[test]
    fn test() {
        assert_eq!(find_low_risk("./inputs/day-15-input.txt").unwrap(), 673);
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            find_big_map_low_risk("./inputs/day-15-input-test.txt").unwrap(),
            315
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            find_big_map_low_risk("./inputs/day-15-input.txt").unwrap(),
            2893
        );
    }
}
