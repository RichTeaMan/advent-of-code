use std::io::{self};
use utils::file_utils::read_lines;

pub fn day_3() -> io::Result<u64> {
    find_high_jolts("./inputs/day-3-input.txt")
}

pub fn day_3_part_2() -> io::Result<u64> {
    find_high_very_high_jolts("./inputs/day-3-input.txt")
}

fn find_high_jolts(filename: &str) -> io::Result<u64> {
    let lines = fetch_lines(filename)?;

    let mut jolt_sum: u64 = 0;

    for line in lines {
        let max = line
            .chars()
            .take(line.len() - 1)
            .map(|c| c.to_digit(10).unwrap())
            .max()
            .unwrap();

        let (_, sub) = line.split_once(&max.to_string()).unwrap();

        let max_unit = sub.chars().map(|c| c.to_digit(10).unwrap()).max().unwrap();

        let jolt = (max * 10) + max_unit;
        jolt_sum += jolt as u64;
    }

    Ok(jolt_sum)
}

fn find_high_very_high_jolts(filename: &str) -> io::Result<u64> {
    let lines = fetch_lines(filename)?;

    let mut jolt_sum: u64 = 0;

    for line in lines {
        let mut part = line.clone();
        let mut digits = Vec::new();

        for n in 0..12 {
            let end = part.len() - (11 - n);

            let max = part
                .chars()
                .take(end)
                .map(|c| c.to_digit(10).unwrap())
                .max()
                .unwrap() as u64;

            let (_, sub) = part.split_once(&max.to_string()).unwrap();

            part = sub.to_string();

            digits.push(max);
        }

        let mut jolt = 0;
        for digit in digits {
            jolt *= 10;
            jolt += digit;
        }

        jolt_sum += jolt;
    }

    Ok(jolt_sum)
}

fn fetch_lines(filename: &str) -> io::Result<Vec<String>> {
    let mut result = Vec::new();

    let lines = read_lines(filename)?;
    for line in lines.map_while(Result::ok) {
        if line.is_empty() {
            continue;
        }
        result.push(line);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            find_high_jolts("./inputs/day-3-input-test.txt").unwrap(),
            357
        );
    }

    #[test]
    fn test() {
        assert_eq!(find_high_jolts("./inputs/day-3-input.txt").unwrap(), 17100);
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            find_high_very_high_jolts("./inputs/day-3-input-test.txt").unwrap(),
            3121910778619
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            find_high_very_high_jolts("./inputs/day-3-input.txt").unwrap(),
            170418192256861
        );
    }
}
