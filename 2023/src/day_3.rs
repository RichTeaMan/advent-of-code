use std::{
    collections::{HashMap, HashSet},
    io::{self},
    option,
};
use utils::{coordinate::Coordinate, file_utils::read_lines};

struct Schematic {
    symbol_coords: HashMap<Coordinate, char>,
    digit_coords: HashMap<Coordinate, i32>,
}

pub fn day_3() -> io::Result<i32> {
    fetch_part_numbers("./inputs/day-3-input.txt")
}
pub fn day_3_part_2() -> io::Result<i32> {
    fetch_gear_ratios("./inputs/day-3-input.txt")
}

fn fetch_digit_from_vec(vec: &[char]) -> option::Option<i32> {
    let res = vec.iter().collect::<String>().parse::<i32>();
    if let Ok(digit) = res {
        Some(digit)
    } else {
        None
    }
}

fn fetch_schematic(filename: &str) -> io::Result<Schematic> {
    let mut symbol_coords = HashMap::new();
    let mut digit_coords = HashMap::new();

    let lines = read_lines(filename)?;
    for (y, line) in lines.flatten().enumerate() {
        if line.is_empty() {
            continue;
        }

        let mut digits = Vec::new();
        for (x, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                digits.push(c.to_owned());
                continue;
            } else if c != '.' {
                // a symbol!
                symbol_coords.insert(Coordinate::new(x as i32, y as i32), c);
            }
            if let Some(number) = fetch_digit_from_vec(&digits) {
                for i in 0..digits.len() {
                    digit_coords.insert(Coordinate::new(((x - i) - 1) as i32, y as i32), number);
                }
                digits.clear();
            }
        }
        if let Some(number) = fetch_digit_from_vec(&digits) {
            for i in 0..digits.len() {
                digit_coords.insert(
                    Coordinate::new(((line.len() - i) - 1) as i32, y as i32),
                    number,
                );
            }
        }
    }
    let schematic = Schematic {
        digit_coords,
        symbol_coords,
    };
    Ok(schematic)
}

fn fetch_part_numbers(filename: &str) -> io::Result<i32> {
    let schematic = fetch_schematic(filename)?;

    let mut part_number_sum = 0;

    for symbol_coord in schematic.symbol_coords.keys() {
        // big assumption here: identical part numbers are never surrounding the same symbol.
        let mut found_parts = HashSet::new();
        for coord in symbol_coord.surround() {
            if let Some(number) = schematic.digit_coords.get(&coord) {
                found_parts.insert(number.to_owned());
            }
        }
        part_number_sum += found_parts.iter().sum::<i32>();
    }
    Ok(part_number_sum)
}

fn fetch_gear_ratios(filename: &str) -> io::Result<i32> {
    let schematic = fetch_schematic(filename)?;

    let mut part_number_sum = 0;

    for (symbol_coord, s) in &schematic.symbol_coords {
        if *s != '*' {
            continue;
        }
        // big assumption here: identical part numbers are never surrounding the same symbol.
        let mut found_parts = HashSet::new();
        for coord in symbol_coord.surround() {
            if let Some(number) = schematic.digit_coords.get(&coord) {
                found_parts.insert(number.to_owned());
            }
        }
        // only counts if there are exactly two parts
        if found_parts.len() != 2 {
            continue;
        }
        part_number_sum += found_parts.iter().product::<i32>();
    }
    Ok(part_number_sum)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            fetch_part_numbers("./inputs/day-3-input-test.txt").unwrap(),
            4361
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            fetch_part_numbers("./inputs/day-3-input.txt").unwrap(),
            528799
        );
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            fetch_gear_ratios("./inputs/day-3-input-test.txt").unwrap(),
            467835
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            fetch_gear_ratios("./inputs/day-3-input.txt").unwrap(),
            84907174
        );
    }
}
