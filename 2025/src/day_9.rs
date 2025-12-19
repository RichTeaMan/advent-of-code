use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    io::{self},
};
use utils::file_utils::read_lines;

pub fn day_9() -> io::Result<u64> {
    calc_connections("./inputs/day-9-input.txt")
}

pub fn day_9_part_2() -> io::Result<u64> {
    calc_shapes("./inputs/day-9-input.txt")
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

fn dist_squared(a: &Position, b: &Position) -> u64 {
    (a.x - b.x).abs().pow(2) as u64 + (a.y - b.y).abs().pow(2) as u64
}

fn calc_connections(filename: &str) -> io::Result<u64> {
    let db = fetch_positions(filename)?;

    let mut dist_sqr = Vec::new();

    for l in db.iter().combinations(2) {
        let (a, b) = l.iter().collect_tuple().unwrap();
        if a == b {
            continue;
        }
        // the reverse will be stored as well. im choosing to be ok with it
        let d = dist_squared(a, b);
        dist_sqr.push((**a, **b, d));
    }
    dist_sqr.sort_by(|a, b| b.2.cmp(&a.2));

    // BIG assumption that largest rectangles must be within set that have largest hypotenuses
    let mut areas = Vec::new();
    for (a, b, _) in &dist_sqr {
        let area = ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1);
        areas.push(area as u64);
    }

    areas.sort_by(|a, b| b.cmp(a));

    let result = *areas.first().unwrap();

    Ok(result)
}

fn quick_calc_row(row: i64, min_x: i64, max_x: i64, tiles_by_row: &HashMap<i64, Vec<i64>>) -> bool {
    let row_tiles = tiles_by_row.get(&row).unwrap();

    if min_x < *row_tiles.first().unwrap() || max_x > *row_tiles.last().unwrap() {
        return false;
    }

    let mut in_shape = false;
    let mut last_opt: Option<i64> = None;
    for xx in row_tiles {
        if xx >= &max_x {
            return in_shape;
        }
        let mut is_finding_border = false;

        if let Some(last) = last_opt {
            if last + 1 == *xx {
                is_finding_border = true;
            }
        }

        if !is_finding_border {
            in_shape = !in_shape;
        }

        last_opt = Some(*xx);
    }
    true
}

fn calc_shapes(filename: &str) -> io::Result<u64> {
    let mut red_tiles: Vec<Position> = fetch_positions(filename)?;
    red_tiles.push(*red_tiles.first().unwrap());

    let mut green_tiles = HashSet::new();
    let mut last_tile_opt: Option<Position> = None;
    for p in &red_tiles {
        if let Some(last_tile) = last_tile_opt {
            // find step
            debug_assert_ne!(last_tile, *p);
            let mut x_step = 0;
            let mut y_step = 0;
            if last_tile.x == p.x {
                if last_tile.y > p.y {
                    y_step = -1;
                }
                if last_tile.y < p.y {
                    y_step = 1;
                }
            }
            if last_tile.y == p.y {
                if last_tile.x > p.x {
                    x_step = -1;
                }
                if last_tile.x < p.x {
                    x_step = 1;
                }
            }
            debug_assert_ne!(x_step, y_step);

            let mut green_tile = last_tile;
            green_tiles.insert(green_tile);
            while green_tile != *p {
                // implicit copy
                green_tiles.insert(green_tile);
                green_tile.x += x_step;
                green_tile.y += y_step;
            }
        }
        last_tile_opt = Some(*p);
        green_tiles.insert(*p);
    }

    let mut tiles_by_row = HashMap::new();
    for tile in &green_tiles {
        tiles_by_row
            .entry(tile.y)
            .and_modify(|t: &mut Vec<i64>| t.push(tile.x))
            .or_insert(vec![tile.x]);
    }
    for v in tiles_by_row.values_mut() {
        v.sort();
    }

    let mut candidates = Vec::new();

    for l in red_tiles.iter().combinations(2) {
        let (a, b) = l.iter().collect_tuple().unwrap();
        if a == b {
            continue;
        }

        let area = ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1);

        candidates.push((**a, **b, area));
    }
    candidates.sort_by(|a, b| b.2.cmp(&a.2));

    let mut areas = Vec::new();
    for (a, b, area) in candidates {
        // must not be a red tile inside, that implies the entire rectangle is not full of greens

        let min_x = a.x.min(b.x);
        let min_y = a.y.min(b.y);
        let max_x = a.x.max(b.x);
        let max_y = a.y.max(b.y);

        let mut red_found = false;
        for p in &red_tiles {
            if p.x > min_x && p.x < max_x && p.y > min_y && p.y < max_y {
                red_found = true;
                break;
            }
        }

        if red_found {
            continue;
        }

        let mut missing_tile = false;
        for y in min_y..=max_y {
            if !quick_calc_row(y, min_x, max_x, &tiles_by_row) {
                missing_tile = true;
                break;
            }
        }

        if !missing_tile {
            areas.push(area);
            break;
        }
    }
    areas.sort();
    Ok(*areas.last().unwrap() as u64)
}

fn fetch_positions(filename: &str) -> io::Result<Vec<Position>> {
    let mut result = Vec::new();

    let lines = read_lines(filename)?;
    for line in lines.map_while(Result::ok) {
        if line.is_empty() {
            continue;
        }
        let (x, y) = line
            .split(',')
            .map(|p| p.parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        let junction_position = Position { x, y };
        result.push(junction_position);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            calc_connections("./inputs/day-9-input-test.txt").unwrap(),
            50
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            calc_connections("./inputs/day-9-input.txt").unwrap(),
            4761736832
        );
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(calc_shapes("./inputs/day-9-input-test.txt").unwrap(), 24);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(calc_shapes("./inputs/day-9-input.txt").unwrap(), 1452422268);
    }
}
