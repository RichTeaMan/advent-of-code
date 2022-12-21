use std::{collections::HashMap, io};

use itertools::Itertools;

use crate::file_utils::read_lines;

struct Monkey {
    pub operation: Operand,
    pub monkey_name_a: String,
    pub monkey_name_b: String,
}

enum Operand {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl Operand {
    pub fn from_str(s: &str) -> Self {
        match s {
            "+" => Self::Plus,
            "-" => Self::Minus,
            "*" => Self::Multiply,
            "/" => Self::Divide,
            _ => panic!("Unknown operation"),
        }
    }

    pub fn calc(&self, a: i64, b: i64) -> i64 {
        match self {
            Operand::Plus => a + b,
            Operand::Multiply => a * b,
            Operand::Minus => a - b,
            Operand::Divide => a / b,
        }
    }
}

pub fn day_21() -> io::Result<i64> {
    let positions = monkey_puzzle("./inputs/day-21-input.txt")?;
    Ok(positions)
}

pub fn day_21_part_2() -> io::Result<i64> {
    todo!();
}

fn monkey_puzzle(filename: &str) -> io::Result<i64> {
    let mut monkey_list = HashMap::new();
    let mut known_numbers = HashMap::new();

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }
        let trim = line.trim();

        if let Some((name, operation)) = trim.split(':').collect_tuple() {
            if operation.contains('+')
                || operation.contains('-')
                || operation.contains('*')
                || operation.contains('/')
            {
                if let Some((a, op, b)) = operation.trim().split(' ').collect_tuple() {
                    let monkey = Monkey {
                        operation: Operand::from_str(op),
                        monkey_name_a: a.to_string(),
                        monkey_name_b: b.to_string(),
                    };
                    monkey_list.insert(name.to_string(), monkey);
                } else {
                    panic!("Bad input.");
                }
            } else {
                println!("{operation}");
                let value = operation.trim().parse::<i64>().unwrap();
                known_numbers.insert(name.to_string(), value);
            }
        } else {
            panic!("Bad input: '{trim}'");
        }
    }

    while monkey_list.contains_key("root") {
        let mut name_to_remove = Vec::new();

        for (name, monkey) in &monkey_list {
            let a_opt = known_numbers.get(&monkey.monkey_name_a);

            if let Some(a) = a_opt {
                let b_opt = known_numbers.get(&monkey.monkey_name_b);

                if let Some(b) = b_opt {
                    let result = monkey.operation.calc(*a, *b);
                    known_numbers.insert(name.clone(), result);

                    name_to_remove.push(name.clone());
                }
            }
        }

        debug_assert!(!name_to_remove.is_empty(), "Could not remove any names.");

        for n in name_to_remove {
            monkey_list.remove(&n);
        }
    }

    Ok(known_numbers["root"])
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(monkey_puzzle("./inputs/day-21-input-test.txt").unwrap(), 152);
    }

    #[test]
    fn test() {
        assert_eq!(
            monkey_puzzle("./inputs/day-21-input.txt").unwrap(),
            142707821472432
        );
    }
}
