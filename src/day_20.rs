use std::{collections::VecDeque, io};

use itertools::Itertools;

use crate::file_utils::read_lines;

const DECRYPTION_KEY: i64 = 811589153;

#[derive(Copy, Clone)]
struct Number {
    pub value: i64,
    pub decrypted: bool,
}

#[allow(dead_code)]
fn print_list(numbers: &VecDeque<Number>) {
    for n in numbers {
        if n.decrypted {
            print!(" {v} , ", v = n.value);
        } else {
            print!("({v}), ", v = n.value);
        }
    }
    println!();
}

fn decrypt(mix_amount: i32, decryption_key: i64, filename: &str) -> io::Result<Vec<i64>> {
    let mut numbers = VecDeque::new();

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }
        let value = line.parse::<i64>().unwrap();
        numbers.push_back(value * decryption_key);
    }

    let mut results = (0..numbers.len()).collect_vec();

    // heavily inspired by AxlLind's code.
    for _ in 0..mix_amount {
        for (i, x) in numbers.iter().enumerate() {
            let index = results.iter().position(|&r| r == i).unwrap();
            results.remove(index);
            let mut new_index = index as i64 + x;
            new_index = new_index.rem_euclid(results.len() as i64);

            debug_assert!(
                new_index >= 0 && new_index < numbers.len() as i64 + 1,
                "{new_index}"
            );

            results.insert(new_index as usize, i as usize);
        }
    }

    Ok(results.iter().map(|&r| numbers[r]).collect_vec())
}

fn find_coordinates(mix_amount: i32, decryption_key: i64, filename: &str) -> io::Result<i64> {
    let data = decrypt(mix_amount, decryption_key, filename)?;

    let length = data.len();
    let z_position = data.iter().position(|n| *n == 0).unwrap();

    let i1000 = (z_position + 1000) % length;
    let i2000 = (z_position + 2000) % length;
    let i3000 = (z_position + 3000) % length;

    let c1000 = data[i1000];
    let c2000 = data[i2000];
    let c3000 = data[i3000];

    let result = c1000 + c2000 + c3000;
    Ok(result)
}

pub fn day_20() -> io::Result<i64> {
    let result = find_coordinates(1, 1, "./inputs/day-20-input.txt")?;
    Ok(result)
}

pub fn day_20_part_2() -> io::Result<i64> {
    let result = find_coordinates(10, DECRYPTION_KEY, "./inputs/day-20-input.txt")?;
    Ok(result)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        let result = find_coordinates(1, 1, "./inputs/day-20-input-test.txt").unwrap();
        assert_eq!(3, result);
    }

    #[test]
    fn test() {
        let result = find_coordinates(1, 1, "./inputs/day-20-input.txt").unwrap();
        assert_eq!(2275, result);
    }

    #[test]
    fn part_2_small_test() {
        let result =
            find_coordinates(10, DECRYPTION_KEY, "./inputs/day-20-input-test.txt").unwrap();
        assert_eq!(1623178306, result);
    }
}
