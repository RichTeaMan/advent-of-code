use std::{
    collections::{HashSet, VecDeque},
    io,
};

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

fn calculate_external_sides(filename: &str) -> io::Result<i32> {
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

    let max_width = cubes.iter().map(|(x, _, _)| x).max().unwrap().to_owned();
    let min_width = cubes.iter().map(|(x, _, _)| x).min().unwrap().to_owned();

    let max_height = cubes.iter().map(|(_, y, _)| y).max().unwrap().to_owned();
    let min_height = cubes.iter().map(|(_, y, _)| y).min().unwrap().to_owned();

    let max_depth = cubes.iter().map(|(_, _, z)| z).max().unwrap().to_owned();
    let min_depth = cubes.iter().map(|(_, _, z)| z).min().unwrap().to_owned();

    let margin = 5;
    let min = min_width.min(min_height.min(min_depth)) - margin;
    let max = max_width.max(max_height.max(max_depth)) + margin + 1;

    let mut cube_fill = HashSet::new();
    for i in min..max {
        for j in min..max {
            cubes.insert((i, j, min));
            cubes.insert((i, j, max));
            cubes.insert((i, min, j));
            cubes.insert((i, max, j));
            cubes.insert((min, i, j));
            cubes.insert((max, i, j));
        }
    }

    fill_cubes(&mut cube_fill, &cubes, (min + 1, min + 1, min + 1));

    let mut inverse = HashSet::new();
    for i in (min + margin)..=(max - margin) {
        for j in (min + margin)..=(max - margin) {
            for k in (min + margin)..=(max - margin) {
                if !cube_fill.contains(&(i, j, k)) {
                    inverse.insert((i, j, k));
                }
            }
        }
    }

    let mut sides = 0;
    for (x, y, z) in &inverse {
        if !inverse.contains(&(x + 1, *y, *z)) {
            println!("{d:?}", d = (x + 1, *y, *z));
            sides += 1;
        }
        if !inverse.contains(&(x - 1, *y, *z)) {
            sides += 1;
        }
        if !inverse.contains(&(*x, y + 1, *z)) {
            sides += 1;
        }
        if !inverse.contains(&(*x, *y - 1, *z)) {
            sides += 1;
        }
        if !inverse.contains(&(*x, *y, z + 1)) {
            sides += 1;
        }
        if !inverse.contains(&(*x, *y, z - 1)) {
            sides += 1;
        }
    }

    Ok(sides)
}

fn fill_cubes(
    cubes: &mut HashSet<(i32, i32, i32)>,
    reference: &HashSet<(i32, i32, i32)>,
    fill_point: (i32, i32, i32),
) {
    let mut stack = VecDeque::new();
    stack.push_back(fill_point);

    while let Some(current_fill_point) = stack.pop_front() {
        if !reference.contains(&current_fill_point) && !cubes.contains(&current_fill_point) {
            cubes.insert(current_fill_point);
            let (x, y, z) = current_fill_point;

            let mut points = Vec::new();

            points.push((x - 1, y, z));
            points.push((x + 1, y, z));
            points.push((x, y - 1, z));
            points.push((x, y + 1, z));
            points.push((x, y, z - 1));
            points.push((x, y, z + 1));

            for p in points {
                if !reference.contains(&p) && !cubes.contains(&p) {
                    stack.push_back(p);
                }
            }
        }
    }
}

pub fn day_18() -> io::Result<i32> {
    let result = calculate_exposed_sides("./inputs/day-18-input.txt")?;
    Ok(result)
}

pub fn day_18_part_2() -> io::Result<i32> {
    let result = calculate_external_sides("./inputs/day-18-input.txt")?;
    Ok(result)
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

    #[test]
    fn part_2_small_test() {
        let result = calculate_external_sides("./inputs/day-18-input-test.txt").unwrap();
        assert_eq!(result, 58);
    }

    #[test]
    fn part_2_test() {
        let result = calculate_external_sides("./inputs/day-18-input.txt").unwrap();
        assert_eq!(result, 1986);
    }
}
