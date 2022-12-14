use std::{collections::HashMap, io};

use itertools::Itertools;

use crate::file_utils::read_lines;

type Map = HashMap<(i32, i32), char>;

const SAND_X: i32 = 500;
const SAND_Y: i32 = 0;

#[allow(dead_code)]
fn draw_map(map: &Map) {
    // find lowest point
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;

    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;

    for (x, y) in map.keys() {
        if x < &min_x {
            min_x = *x;
        }
        if x > &max_x {
            max_x = *x;
        }

        if y < &min_y {
            min_y = *y;
        }
        if y > &max_y {
            max_y = *y
        }
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if let Some(c) = map.get(&(x, y)) {
                print!("{c}");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn read_slice_from_file(filename: &str) -> io::Result<Map> {
    let lines = read_lines(filename)?;

    let mut map: Map = HashMap::new();
    map.insert((SAND_X, SAND_Y), '+');
    for line in lines.flatten() {

        let mut prev_coord = None;
        let parts = line.split("->");
        for part in parts {
            if let Some((x, y)) = part.trim().split(',').collect_tuple() {
                let nx = x.parse::<i32>().unwrap();
                let ny = y.parse::<i32>().unwrap();
                if let Some((prev_x, prev_y)) = prev_coord {
                    let mut x_step = 1;
                    let mut x_limit = nx + 1;
                    if prev_x > nx {
                        x_step = -1;
                        x_limit = nx - 1;
                    }
                    let mut y_step = 1;
                    let mut y_limit = ny + 1;
                    if prev_y > ny {
                        y_step = -1;
                        y_limit = ny - 1;
                    }

                    let mut i = prev_x;
                    while i != x_limit {
                        let mut j = prev_y;
                        while j != y_limit {
                            map.insert((i, j), '#');
                            j += y_step;
                        }
                        i += x_step;
                    }
                }
                prev_coord = Some((nx, ny));
            } else {
                panic!("Failed to parse.");
            }
        }
    }
    Ok(map)
}

fn sand_simulation(map: &mut Map) -> i32 {
    let mut result = 0;
    let mut max_y = i32::MIN;

    for (_, y) in map.keys() {
        if y > &max_y {
            max_y = *y
        }
    }

    for _ in 0..1_000_000 {
        let mut active_sand_x = SAND_X;
        let mut active_sand_y = SAND_Y;
        loop {
            if active_sand_y > max_y {
                // off bottom of map
                return result;
            }

            // check down
            if !map.contains_key(&(active_sand_x, active_sand_y + 1)) {
                active_sand_y += 1;
            }
            // check down and left
            else if !map.contains_key(&(active_sand_x - 1, active_sand_y + 1)) {
                active_sand_x -= 1;
                active_sand_y += 1;
            }
            // check down and right
            else if !map.contains_key(&(active_sand_x + 1, active_sand_y + 1)) {
                active_sand_x += 1;
                active_sand_y += 1;
            } else {
                // sand is settled
                result += 1;
                map.insert((active_sand_x, active_sand_y), 'o');

                break;
            }
        }
    }
    panic!("Sand simulation took too long.");
}

pub fn day_14() -> io::Result<i32> {
    let mut map = read_slice_from_file("./inputs/day-14-input.txt")?;
    let result = sand_simulation(&mut map);
    Ok(result)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        let mut map = read_slice_from_file("./inputs/day-14-input-test.txt").unwrap();
        let result = sand_simulation(&mut map);
        draw_map(&map);
        assert_eq!(result, 24);
    }

    #[test]
    fn test() {
        let mut map = read_slice_from_file("./inputs/day-14-input.txt").unwrap();
        let result = sand_simulation(&mut map);
        draw_map(&map);
        assert_eq!(result, 618);
    }
}
