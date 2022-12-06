use std::io::{self};

use itertools::Itertools;
use slice_group_by::GroupBy;

use crate::file_utils::read_lines;

pub fn day_3() -> io::Result<i32> {
    fetch_item_priorities("./inputs/day-3-input.txt")
}

pub fn day_3_part_2() -> io::Result<i32> {
    fetch_group_priorities("./inputs/day-3-input.txt")
}

fn fetch_item_priorities(filename: &str) -> io::Result<i32> {
    let mut priority_sum = 0;

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }
        let split_position = line.len() / 2;
        let (first, second) = line.split_at(split_position);

        assert_eq!(first.len(), second.len());

        for letter in first.chars().unique() {
            if second.contains(letter) {
                priority_sum += fetch_priority(letter);
            }
        }
    }
    Ok(priority_sum)
}

fn fetch_group_priorities(filename: &str) -> io::Result<i32> {
    let mut priority_sum = 0;

    let lines = read_lines(filename)?;
    let mut letters = Vec::new();
    let mut group_count = 0;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }

        letters.extend(line.chars().unique());
        group_count += 1;
        if group_count < 3 {
            continue;
        }

        letters.sort();
        let mut badge = None;
        for group in letters.linear_group_by(|a, b| a == b) {
            if group.len() == 3 {
                assert!(badge.is_none());
                badge = Some(group[0]);
            }
        }

        if let Some(group_letter) = badge {
            priority_sum += fetch_priority(group_letter);
        } else {
            panic!("No group found.");
        }

        letters.clear();
        group_count = 0;
    }
    Ok(priority_sum)
}

fn fetch_priority(letter: char) -> i32 {
    if letter.is_ascii_uppercase() {
        (letter as i32) - 38
    } else {
        (letter as i32) - 96
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            fetch_item_priorities("./inputs/day-3-input-test.txt").unwrap(),
            157
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            fetch_group_priorities("./inputs/day-3-input-test.txt").unwrap(),
            70
        );
    }

    #[test]
    fn fetch_priority_test() {
        assert_eq!(fetch_priority('a'), 1);
        assert_eq!(fetch_priority('z'), 26);
        assert_eq!(fetch_priority('A'), 27);
        assert_eq!(fetch_priority('Z'), 52);
    }
}
