use std::{
    collections::VecDeque,
    io,
};

use itertools::Itertools;

use crate::file_utils::read_lines;

#[derive(Copy, Clone)]
struct Number {
    pub value: i32,
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

fn decrypt(filename: &str) -> io::Result<Vec<i32>> {
    let mut numbers = VecDeque::new();
    let mut stack = VecDeque::new();

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }
        let value = line.parse::<i32>().unwrap();
        numbers.push_back(Number {
            value,
            decrypted: false,
        });

        stack.push_back(value);
    }

    let length = numbers.len();

    let mut i = 0;
    while i < length {
        if numbers[i].decrypted {
            i += 1;
            continue;
        }
        let pulled_opt = numbers.remove(i);
        let mut pulled = pulled_opt.unwrap();
        let mut new_index = i as i32 + pulled.value;

        while new_index <= 0 {
            new_index += length as i32 - 1;
        }

        while new_index >= length as i32 {
            new_index -= length as i32 - 1;
        }
        debug_assert!(new_index >= 0 && new_index < numbers.len() as i32 + 1, "{new_index}");

        pulled.decrypted = true;

        numbers.insert(new_index as usize, pulled);
    }

    let result = numbers.iter().map(|n| n.value).collect_vec();

    Ok(result)
}

fn find_coordinates(filename: &str) -> io::Result<i32> {
    let data = decrypt(filename)?;

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

pub fn day_20() -> io::Result<i32> {
    let result = find_coordinates("./inputs/day-20-input.txt")?;
    Ok(result)
}

pub fn day_20_part_2() -> io::Result<i32> {
    let result = decrypt("./inputs/day-20-input.txt")?;
    todo!();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn collection_test() {
        let result = decrypt("./inputs/day-20-input-test.txt").unwrap();
        let expected = vec![1, 2, -3, 4, 0, 3, -2];
        assert_eq!(expected, result);
    }

    #[test]
    fn small_test() {
        let result = find_coordinates("./inputs/day-20-input-test.txt").unwrap();
        assert_eq!(3, result);
    }

    #[test]
    fn test() {
        let result = find_coordinates("./inputs/day-20-input.txt").unwrap();
        assert_eq!(2275, result);
    }
}

