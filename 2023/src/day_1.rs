use std::io::{self};
use utils::file_utils::read_lines;

const WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn day_1() -> io::Result<i32> {
    fetch_digits("./inputs/day-1-input.txt")
}
pub fn day_1_part_2() -> io::Result<i32> {
    fetch_digits_and_words("./inputs/day-1-input.txt")
}

fn read_digits(line: &str) -> i32 {
    let mut result = 0;
    // read from left
    for c in line.chars() {
        if c.is_ascii_digit() {
            result = c.to_string().parse::<i32>().unwrap() * 10;
            break;
        }
    }

    // read from right
    for c in line.chars().rev() {
        if c.is_ascii_digit() {
            result += c.to_string().parse::<i32>().unwrap();
            break;
        }
    }
    result
}

fn read_digits_and_words(line: &str) -> i32 {
    let mut result = 0;

    let mut mod_line = line.to_string();

    for (i, w) in WORDS.iter().enumerate() {
        let digit = i + 1;
        let a = w.chars().next().unwrap();
        let c = w.chars().last().unwrap();
        let rep = format!("{a}{digit}{c}");

        mod_line = mod_line.replace(w, &rep);
    }

    // read from left
    for c in mod_line.chars() {
        if c.is_ascii_digit() {
            result = c.to_string().parse::<i32>().unwrap() * 10;
            break;
        }
    }

    // read from right
    for c in mod_line.chars().rev() {
        if c.is_ascii_digit() {
            result += c.to_string().parse::<i32>().unwrap();
            break;
        }
    }
    result
}

fn fetch_digits(filename: &str) -> io::Result<i32> {
    let mut result = 0;

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }
        result += read_digits(line.as_str());
    }
    Ok(result)
}

fn fetch_digits_and_words(filename: &str) -> io::Result<i32> {
    let mut result = 0;

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }
        result += read_digits_and_words(line.as_str());
    }
    Ok(result)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(fetch_digits("./inputs/day-1-input-test.txt").unwrap(), 142);
    }

    #[test]
    fn test() {
        assert_eq!(fetch_digits("./inputs/day-1-input.txt").unwrap(), 54644);
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            fetch_digits_and_words("./inputs/day-1-input-test-2.txt").unwrap(),
            281
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            fetch_digits_and_words("./inputs/day-1-input.txt").unwrap(),
            53348
        );
    }
}
