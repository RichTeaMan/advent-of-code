use std::io::{self};

use itertools::Itertools;

use utils::file_utils::read_lines;

pub fn day_2() -> io::Result<i32> {
    calc_position("./inputs/day-2-input.txt")
}
pub fn day_2_part_2() -> io::Result<i32> {
    calc_position_with_aim("./inputs/day-2-input.txt")
}

fn calc_position(filename: &str) -> io::Result<i32> {
    let mut x = 0;
    let mut y = 0;
    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if let Some((direction, distance_str)) = line.split(' ').collect_tuple() {
            if let Ok(distance) = distance_str.parse::<i32>() {
                match direction {
                    "forward" => x += distance,
                    "down" => y += distance,
                    "up" => y -= distance,
                    x => panic!("Unknown direction '{x}'"),
                };
            } else {
                panic!("Could not parse distance. {dist}", dist = distance_str);
            }
        }
    }
    Ok(x * y)
}

fn calc_position_with_aim(filename: &str) -> io::Result<i32> {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if let Some((direction, distance_str)) = line.split(' ').collect_tuple() {
            if let Ok(distance) = distance_str.parse::<i32>() {
                match direction {
                    "forward" => {
                        x += distance;
                        y += aim * distance
                    }
                    "down" => aim += distance,
                    "up" => aim -= distance,
                    x => panic!("Unknown direction '{x}'"),
                };
            } else {
                panic!("Could not parse distance. {dist}", dist = distance_str);
            }
        }
    }
    Ok(x * y)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(calc_position("./inputs/day-2-input-test.txt").unwrap(), 150);
    }

    #[test]
    fn test() {
        assert_eq!(calc_position("./inputs/day-2-input.txt").unwrap(), 2102357);
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            calc_position_with_aim("./inputs/day-2-input-test.txt").unwrap(),
            900
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            calc_position_with_aim("./inputs/day-2-input.txt").unwrap(),
            2101031224
        );
    }
}
