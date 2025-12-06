use itertools::Itertools;
use std::io::{self};
use utils::file_utils::read_lines;

pub fn day_6() -> io::Result<u64> {
    calc_answers("./inputs/day-6-input.txt")
}

pub fn day_6_part_2() -> io::Result<u64> {
    fetch_ceph_math("./inputs/day-6-input.txt")
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MathQuestions {
    math: Vec<Vec<u64>>,
    operators: Vec<char>,
}

fn calc_answers(filename: &str) -> io::Result<u64> {
    let db = fetch_math(filename)?;

    let math_lengths = db.math.iter().map(|m| m.len()).collect_vec();
    let problem_count = *math_lengths.iter().max().unwrap();
    debug_assert_eq!(*math_lengths.iter().min().unwrap(), problem_count);
    debug_assert_eq!(db.operators.len(), problem_count);

    let mut result = 0;
    for i in 0..problem_count {
        let nums: Vec<u64> = db
            .math
            .iter()
            .map(|l| *l.get(i).unwrap())
            .collect_vec();

        let answer: u64 = match db.operators.get(i).unwrap() {
            '+' => nums.iter().sum(),
            '*' => nums.iter().product(),
            _ => panic!("Unknown op"),
        };
        result += answer;
    }

    Ok(result)
}

fn fetch_math(filename: &str) -> io::Result<MathQuestions> {
    let mut result = MathQuestions {
        math: Vec::new(),
        operators: Vec::new(),
    };

    let lines = read_lines(filename)?;
    for line in lines.map_while(Result::ok) {
        if line.is_empty() {
            continue;
        }

        let parts = line.split(' ').filter(|p| !p.is_empty());

        if line.contains('+') {
            for p in parts {
                let op = match p {
                    "+" => '+',
                    "*" => '*',
                    _ => panic!("Unknown operator"),
                };
                result.operators.push(op);
            }
        } else {
            let numbers = parts.map(|p| p.parse::<u64>().unwrap()).collect_vec();
            result.math.push(numbers);
        }
    }
    Ok(result)
}

fn fetch_ceph_math(filename: &str) -> io::Result<u64> {
    let mut num_lines = Vec::new();
    let mut op_line: String = "".to_string();

    let lines = read_lines(filename)?;
    for line in lines.map_while(Result::ok) {
        if line.is_empty() {
            continue;
        }

        if line.contains('+') {
            op_line = line;
        } else {
            num_lines.push(line.chars().map(|c| c.to_digit(10)).collect_vec());
        }
    }

    let mut values = Vec::new();
    let mut result = 0;
    let op_vec = op_line.chars().collect_vec();
    for i in (0..op_vec.len()).rev() {
        let c = *op_vec.get(i).unwrap();

        let mut value = 0_u64;

        for num_line in &num_lines {
            let digit_opt = num_line.get(i).unwrap();

            if let Some(digit) = digit_opt {
                value *= 10;
                value += *digit as u64;
            }
        }
        if value > 0 {
            values.push(value);
        }

        if c != ' ' {
            let answer: u64 = match c {
                '+' => values.iter().sum(),
                '*' => values.iter().product(),
                _ => panic!("Unknown op"),
            };
            values.clear();
            result += answer;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            calc_answers("./inputs/day-6-input-test.txt").unwrap(),
            4277556
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            calc_answers("./inputs/day-6-input.txt").unwrap(),
            4580995422905
        );
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            fetch_ceph_math("./inputs/day-6-input-test.txt").unwrap(),
            3263827
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            fetch_ceph_math("./inputs/day-6-input.txt").unwrap(),
            10875057285868
        );
    }
}
