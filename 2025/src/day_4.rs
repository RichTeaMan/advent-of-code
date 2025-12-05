use std::{
    collections::HashSet,
    io::{self},
};
use utils::{coordinate::Coordinate, file_utils::read_lines};

pub fn day_4() -> io::Result<u64> {
    find_rolls("./inputs/day-4-input.txt")
}

pub fn day_4_part_2() -> io::Result<u64> {
    find_and_remove_rolls("./inputs/day-4-input.txt")
}

fn find_rolls(filename: &str) -> io::Result<u64> {
    let coords = fetch_lines(filename)?;

    let mut rolls = 0;
    for coord in &coords {
        if coord
            .surround()
            .iter()
            .filter(|c| coords.contains(c))
            .count()
            < 4
        {
            rolls += 1;
        }
    }
    Ok(rolls)
}

fn find_and_remove_rolls(filename: &str) -> io::Result<u64> {
    let mut coords = fetch_lines(filename)?;

    let mut rolls = 0;
    loop {
        let mut new_coords = HashSet::new();
        for coord in &coords {
            if coord
                .surround()
                .iter()
                .filter(|c| coords.contains(c))
                .count()
                < 4
            {
                rolls += 1;
            }
            else {
                new_coords.insert(*coord);
            }
        }
        if coords.len() == new_coords.len() {
            break;
        }
        coords = new_coords;
    }
    Ok(rolls)
}

fn fetch_lines(filename: &str) -> io::Result<HashSet<Coordinate>> {
    let mut result = HashSet::new();

    let lines = read_lines(filename)?;
    let mut y = 0;
    for line in lines.map_while(Result::ok) {
        if line.is_empty() {
            continue;
        }
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                result.insert(Coordinate { x: x as i32, y });
            }
        }
        y += 1;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(find_rolls("./inputs/day-4-input-test.txt").unwrap(), 13);
    }

    #[test]
    fn test() {
        assert_eq!(find_rolls("./inputs/day-4-input.txt").unwrap(), 1411);
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            find_and_remove_rolls("./inputs/day-4-input-test.txt").unwrap(),
            43
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            find_and_remove_rolls("./inputs/day-4-input.txt").unwrap(),
            8557
        );
    }
}
