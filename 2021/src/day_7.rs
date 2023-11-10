use std::io::{self};

use utils::file_utils::read_lines;

pub fn day_7() -> io::Result<i32> {
    find_low_fuel_position("./inputs/day-7-input.txt")
}
pub fn day_7_part_2() -> io::Result<i32> {
    find_low_triangle_fuel_position("./inputs/day-7-input.txt")
}

fn load_positions(filename: &str) -> io::Result<Vec<i32>> {
    let mut positions = Vec::new();

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }

        for split in line.split(',') {
            let position = split.parse::<i32>().unwrap();
            positions.push(position);
        }
    }
    Ok(positions)
}

fn calc_fuel(positions: Vec<i32>) -> Vec<i32> {
    let max = positions.iter().max().unwrap().to_owned();

    let mut fuels = Vec::new();
    for pos in 0..=max {
        let mut fuel = 0;
        for sub in &positions {
            let diff = (pos - sub).abs();
            fuel += diff;
        }
        fuels.push(fuel);
    }

    fuels
}

fn calc_triangle_fuel(positions: Vec<i32>) -> Vec<i32> {
    let max = positions.iter().max().unwrap().to_owned();

    let mut triangle_nums = Vec::new();
    let mut prev = 0;
    // pre calc triangle numbers
    for i in 0..=max {
        prev += i;
        triangle_nums.push(prev);
    }

    let mut fuels = Vec::new();
    for pos in 0..=max {
        let mut fuel = 0;
        for sub in &positions {
            let diff = (pos - sub).unsigned_abs() as usize;
            fuel += triangle_nums.get(diff).unwrap();
        }
        fuels.push(fuel);
    }

    fuels
}

fn find_low_fuel_position(filename: &str) -> io::Result<i32> {
    let position = load_positions(filename)?;
    let fuels = calc_fuel(position);
    Ok(fuels.iter().min().unwrap().to_owned())
}

fn find_low_triangle_fuel_position(filename: &str) -> io::Result<i32> {
    let position = load_positions(filename)?;
    let fuels = calc_triangle_fuel(position);
    Ok(fuels.iter().min().unwrap().to_owned())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            find_low_fuel_position("./inputs/day-7-input-test.txt").unwrap(),
            37
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            find_low_fuel_position("./inputs/day-7-input.txt").unwrap(),
            352707
        );
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            find_low_triangle_fuel_position("./inputs/day-7-input-test.txt").unwrap(),
            168
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            find_low_triangle_fuel_position("./inputs/day-7-input.txt").unwrap(),
            95519693
        );
    }
}
