use std::io::{self};

use crate::file_utils::read_lines;

/**
 * Gets the highest calorie count from day-1-input.txt.
 */
pub fn day_1() -> io::Result<i32> {
    let mut calories = fetch_calories()?;
    calories.sort();
    calories.reverse();
    Ok(*calories.first().unwrap())
}

/**
 * Gets the sum of the 3 highest calorie counts from day-1-input.txt.
 */
pub fn day_1_part_2() -> io::Result<i32> {
    let mut calories = fetch_calories()?;
    calories.sort();
    calories.reverse();
    Ok(calories.iter().take(3).sum::<i32>())
}

fn fetch_calories() -> io::Result<Vec<i32>> {
    let mut calories_vec = Vec::new();

    let lines = read_lines("./inputs/day-1-input.txt")?;
    let mut calories = 0;
    for line in lines.flatten() {
        let line_calories_opt = line.parse::<i32>();
        if let Ok(line_calories) = line_calories_opt {
            calories += line_calories;
        } else if line.is_empty() {
            calories_vec.push(calories);
            calories = 0;
        }
    }
    Ok(calories_vec)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        assert_eq!(day_1().unwrap(), 70116);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(day_1_part_2().unwrap(), 206582);
    }
}
