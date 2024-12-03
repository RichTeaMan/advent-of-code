use itertools::Itertools;
use std::{
    collections::HashMap,
    io::{self},
};
use utils::file_utils::read_lines;

pub fn day_1() -> io::Result<i32> {
    calc_total_distance("./inputs/day-1-input.txt")
}
pub fn day_1_part_2() -> io::Result<i32> {
    calc_sim_score("./inputs/day-1-input.txt")
}

fn calc_total_distance(line: &str) -> io::Result<i32> {
    let digits = fetch_digits(line).unwrap();

    let mut list_1 = vec![];
    let mut list_2 = vec![];

    for d in digits.iter() {
        list_1.push(d.0);
        list_2.push(d.1);
    }

    list_1.sort();
    list_2.sort();

    let mut dist = 0;
    for i in 0..list_1.len() {
        let len = (list_1[i] - list_2[i]).abs();
        dist += len;
    }

    Ok(dist)
}

fn calc_sim_score(line: &str) -> io::Result<i32> {
    let digits = fetch_digits(line).unwrap();

    let mut list_1 = vec![];
    let mut list_2 = vec![];

    for d in digits.iter() {
        list_1.push(d.0);
        list_2.push(d.1);
    }

    let mut list_2_map = HashMap::new();
    for e in list_2.iter() {
        list_2_map.entry(*e).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut score = 0;
    for e in list_1.iter() {
        if let Some(v) = list_2_map.get(e) {
            score += e * v;
        }
    }

    Ok(score)
}

fn fetch_digits(filename: &str) -> io::Result<Vec<(i32, i32)>> {
    let lines = read_lines(filename)?;
    let mut elements = vec![];
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }
        let mut line_processed = false;
        if let Some((e1_str, e2_str)) = line.split(' ').filter(|&s| !s.is_empty()).collect_tuple() {
            if let Ok(e1) = e1_str.parse::<i32>() {
                if let Ok(e2) = e2_str.parse::<i32>() {
                    elements.push((e1, e2));
                    line_processed = true;
                }
            }
        }
        if !line_processed {
            panic!("Failed to process line: {line}");
        }
    }
    Ok(elements)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            calc_total_distance("./inputs/day-1-input-test.txt").unwrap(),
            11
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            calc_total_distance("./inputs/day-1-input.txt").unwrap(),
            1388114
        );
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(calc_sim_score("./inputs/day-1-input-test.txt").unwrap(), 31);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            calc_sim_score("./inputs/day-1-input.txt").unwrap(),
            23529853
        );
    }
}
