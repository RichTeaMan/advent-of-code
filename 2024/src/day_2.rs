use itertools::Itertools;
use std::io::{self};
use utils::file_utils::read_lines;

pub fn day_2() -> io::Result<i32> {
    calc_safe_reports("./inputs/day-2-input.txt")
}
pub fn day_2_part_2() -> io::Result<i32> {
    calc_dampened_safe_reports("./inputs/day-2-input.txt")
}

fn calc_safe_reports(line: &str) -> io::Result<i32> {
    let reports = fetch_reports(line).unwrap();

    let mut safe_reports = 0;
    for report in reports {
        if evalulate_report(&report) {
            safe_reports += 1;
        }
    }
    Ok(safe_reports)
}

fn calc_dampened_safe_reports(line: &str) -> io::Result<i32> {
    let reports = fetch_reports(line).unwrap();

    let mut safe_reports = 0;
    for report in reports {
        if evalulate_report(&report) {
            safe_reports += 1;
            continue;
        }
        for i in 0..report.len() {
            let mut fudged_report = report.clone();
            fudged_report.remove(i);
            if evalulate_report(&fudged_report) {
                safe_reports += 1;
                break;
            }
        }
    }
    Ok(safe_reports)
}

fn evalulate_report(report: &[i32]) -> bool {
    let increasing = report[0] < *report.last().unwrap();
    let mut current_level = report[0];
    let mut is_safe = true;
    for l in report.iter().skip(1) {
        let diff = (current_level - l).abs();
        if diff == 0 || diff > 3 {
            is_safe = false;
            break;
        }
        if (l > &current_level) != increasing {
            is_safe = false;
            break;
        }
        current_level = *l;
    }
    is_safe
}

fn fetch_reports(filename: &str) -> io::Result<Vec<Vec<i32>>> {
    let lines = read_lines(filename)?;
    let mut reports = vec![];
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let report = line
            .split(' ')
            .filter(|&s| !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap())
            .collect_vec();
        reports.push(report);
    }
    Ok(reports)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            calc_safe_reports("./inputs/day-2-input-test.txt").unwrap(),
            2
        );
    }

    #[test]
    fn test() {
        assert_eq!(calc_safe_reports("./inputs/day-2-input.txt").unwrap(), 472);
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            calc_dampened_safe_reports("./inputs/day-2-input-test.txt").unwrap(),
            4
        );
    }
    
    #[test]
    fn part_2_test() {
        assert_eq!(
            calc_dampened_safe_reports("./inputs/day-2-input.txt").unwrap(),
            520
        );
    }
}
