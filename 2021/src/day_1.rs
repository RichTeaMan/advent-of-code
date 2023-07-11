use std::{
    collections::VecDeque,
    io::{self},
};

use crate::file_utils::read_lines;

pub fn day_1() -> io::Result<i32> {
    fetch_increase_measurements("./inputs/day-1-input.txt")
}
pub fn day_1_part_2() -> io::Result<i32> {
    calc_measurement_window_increase("./inputs/day-1-input.txt")
}

fn fetch_increase_measurements(filename: &str) -> io::Result<i32> {
    let measurements = fetch_measurements(filename)?;

    let mut increases = 0;
    let mut prev_measurement_opt = Option::None;
    for m in &measurements {
        if let Some(prev_measurement) = prev_measurement_opt {
            if m > prev_measurement {
                increases += 1;
            }
        }
        prev_measurement_opt = Some(m);
    }
    Ok(increases)
}

fn calc_measurement_window_increase(filename: &str) -> io::Result<i32> {
    let measurements = fetch_measurements(filename)?;

    let mut increases = 0;
    let mut window = VecDeque::new();
    let mut prev_measurement_opt = Option::None;
    for m in &measurements {
        window.push_back(m.to_owned());
        if window.len() > 3 {
            window.pop_front();
        }

        if window.len() == 3 {
            let measurement: i32 = window.iter().sum::<i32>();

            if let Some(prev_measurement) = prev_measurement_opt {
                if measurement > prev_measurement {
                    increases += 1;
                }
            }
            prev_measurement_opt = Some(measurement);
        }
    }
    Ok(increases)
}

fn fetch_measurements(filename: &str) -> io::Result<Vec<i32>> {
    let mut measurements_vec = Vec::new();

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        let line_measurement_opt = line.parse::<i32>();
        if let Ok(line_measurement) = line_measurement_opt {
            measurements_vec.push(line_measurement);
        }
    }
    Ok(measurements_vec)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            fetch_increase_measurements("./inputs/day-1-input-test.txt").unwrap(),
            7
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            fetch_increase_measurements("./inputs/day-1-input.txt").unwrap(),
            1466
        );
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            calc_measurement_window_increase("./inputs/day-1-input-test.txt").unwrap(),
            5
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            calc_measurement_window_increase("./inputs/day-1-input.txt").unwrap(),
            1491
        );
    }
}
