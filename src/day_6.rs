use std::io::{self};

use itertools::Itertools;

use crate::file_utils::read_lines;

pub fn day_6() -> io::Result<i32> {
    fetch_pattern_position("day-6-input.txt")
}

pub fn day_6_part_2() -> io::Result<i32> {
    todo!();
}

fn fetch_pattern_position(filename: &str) -> io::Result<i32> {

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        let pattern_length = 4;
        for (n, _) in line
            .chars()
            .into_iter()
            .skip(pattern_length - 1)
            .enumerate()
        {
            let sub = &line[n..(n + pattern_length)];
            if sub.chars().unique().count() == 4 {
                return Ok((n + pattern_length) as i32);
            }
        }
    }
    return Ok(-1);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            fetch_pattern_position("./day-6-input-test.txt").unwrap(),
            7
        );
    }

    #[test]
    fn test() {
        assert_eq!(fetch_pattern_position("./day-6-input.txt").unwrap(), 1804);
    }
}
