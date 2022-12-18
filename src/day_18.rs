use std::{collections::HashSet, io};

use itertools::Itertools;

use crate::file_utils::read_lines;

fn calculate_exposed_sides(filename: &str) -> io::Result<i32> {
    let mut cubes = HashSet::new();

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }

        if let Some((x_s, y_s, z_s)) = line.split(',').collect_tuple() {
            let x = x_s.parse::<i32>().unwrap();
            let y = y_s.parse::<i32>().unwrap();
            let z = z_s.parse::<i32>().unwrap();

            cubes.insert((x, y, z));
        } else {
            panic!("Bad input: {line}");
        }
    }

    let mut sides = 0;
    for (x, y, z) in &cubes {
        if !cubes.contains(&(x + 1, *y, *z)) {
            sides += 1;
        }
        if !cubes.contains(&(x - 1, *y, *z)) {
            sides += 1;
        }

        if !cubes.contains(&(*x, y + 1, *z)) {
            sides += 1;
        }
        if !cubes.contains(&(*x, *y - 1, *z)) {
            sides += 1;
        }

        if !cubes.contains(&(*x, *y, z + 1)) {
            sides += 1;
        }
        if !cubes.contains(&(*x, *y, z - 1)) {
            sides += 1;
        }
    }

    Ok(sides)
}

pub fn day_18() -> io::Result<i32> {
    let result = calculate_exposed_sides("./inputs/day-18-input.txt")?;
    Ok(result)
}

pub fn day_18_part_2() -> io::Result<u64> {
    todo!();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        let result = calculate_exposed_sides("./inputs/day-18-input-test.txt").unwrap();
        assert_eq!(result, 64);
    }

    #[test]
    fn test() {
        let result = calculate_exposed_sides("./inputs/day-18-input.txt").unwrap();
        assert_eq!(result, 3470);
    }
}
