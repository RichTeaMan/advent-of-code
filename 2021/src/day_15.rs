use std::io::{self};

use utils::{
    coordinate::{Coordinate, CoordinateMap},
    dijkstra::calc_route_cost,
    file_utils::read_lines,
};

pub fn day_15() -> io::Result<i32> {
    find_low_risk("./inputs/day-15-input.txt")
}
pub fn day_15_part_2() -> io::Result<i32> {
    find_big_map_low_risk("./inputs/day-15-input.txt")
}

fn load_map(filename: &str) -> io::Result<CoordinateMap<i32>> {
    let mut map = CoordinateMap::new();

    let lines = read_lines(filename)?;
    let mut y = 0;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }
        for (x, c) in line.trim().chars().enumerate() {
            let risk = format!("{c}").parse::<i32>().unwrap();

            map.insert(Coordinate::new(x as i32, y), risk);
        }
        y += 1;
    }
    Ok(map)
}

fn load_big_map(filename: &str) -> io::Result<CoordinateMap<i32>> {
    let tile = load_map(filename)?;

    let mut map = CoordinateMap::new();

    let width = tile.keys().map(|c| c.x).max().unwrap().to_owned() + 1;
    let height = tile.keys().map(|c| c.y).max().unwrap().to_owned() + 1;

    debug_assert_eq!(0, width % 10);
    debug_assert_eq!(0, height % 10);

    for i in 0..5 {
        for j in 0..5 {
            let risk_increase = i + j;

            for (tile, risk) in &tile {
                let x = (i * width) + tile.x;
                let y = (j * height) + tile.y;
                debug_assert!(!map.contains_key(&Coordinate::new(x, y)));

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
                map.insert(Coordinate::new(x, y), new_risk);
            }
        }
    }

    Ok(map)
}

fn find_low_risk(filepath: &str) -> io::Result<i32> {
    let map = load_map(filepath)?;
    Ok(find_low_risk_of_map(&map))
}

fn find_big_map_low_risk(filepath: &str) -> io::Result<i32> {
    let map = load_big_map(filepath)?;
    Ok(find_low_risk_of_map(&map))
}

fn find_low_risk_of_map(map: &CoordinateMap<i32>) -> i32 {
    let start = Coordinate::origin();
    let end_x = map.keys().map(|c| c.x).max().unwrap().to_owned();
    let end_y = map.keys().map(|c| c.y).max().unwrap().to_owned();
    let end = Coordinate::new(end_x, end_y);

    calc_route_cost(map, &start, &end)
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
