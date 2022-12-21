use std::{collections::HashMap, io};

use itertools::Itertools;

use crate::file_utils::read_lines;

const ROOT_NAME: &str = "root";
const HUMAN_NAME: &str = "humn";

#[derive(Clone)]
struct Monkey {
    pub operation: Operand,
    pub monkey_name_a: String,
    pub monkey_name_b: String,
}

#[derive(Copy, Clone)]
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
    let result = monkey_puzzle(false, "./inputs/day-21-input.txt")?;
    Ok(result)
}

pub fn day_21_part_2() -> io::Result<i64> {
    let result = monkey_puzzle(true, "./inputs/day-21-input.txt")?;
    Ok(result)
}

fn monkey_puzzle(human_player: bool, filename: &str) -> io::Result<i64> {
    let mut monkey_list = HashMap::new();
    let mut known_numbers = HashMap::new();

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }
        let trim = line.trim();

        if let Some((name, operation)) = trim.split(':').collect_tuple() {
            if human_player && name == HUMAN_NAME {
                continue;
            }

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
                let value = operation.trim().parse::<i64>().unwrap();
                known_numbers.insert(name.to_string(), value);
            }
        } else {
            panic!("Bad input: '{trim}'");
        }
    }

    while monkey_list.contains_key(ROOT_NAME) {
        let mut name_to_remove = Vec::new();

        for (name, monkey) in &monkey_list {
            if human_player && name == HUMAN_NAME {
                continue;
            }
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

        if human_player && name_to_remove.is_empty() {
            let human_answer = algebra_human_answer(&mut monkey_list, &mut known_numbers);
            known_numbers.insert(HUMAN_NAME.to_string(), human_answer);
        }

        for n in name_to_remove {
            monkey_list.remove(&n);
        }
    }

    if human_player {
        Ok(known_numbers[HUMAN_NAME])
    } else {
        Ok(known_numbers[ROOT_NAME])
    }
}

fn some_solve(
    node_name: String,
    should_equal: i64,
    source_monkey_list: &mut HashMap<String, Monkey>,
    source_known_numbers: &mut HashMap<String, i64>,
) {
    if node_name == HUMAN_NAME {
        source_known_numbers.insert(HUMAN_NAME.to_string(), should_equal);
        return;
    }

    let node_a = &source_monkey_list[&node_name].monkey_name_a;
    let node_b = &source_monkey_list[&node_name].monkey_name_b;

    let value_a_opt = source_known_numbers.get(node_a);
    let value_b_opt = source_known_numbers.get(node_b);

    debug_assert!(
        !(value_a_opt.is_some() && value_b_opt.is_some()),
        "Both values are known"
    );
    debug_assert!(
        value_a_opt.is_some() || value_b_opt.is_some(),
        "Both values are unknown"
    );

    let known = value_a_opt.unwrap_or_else(|| value_b_opt.unwrap());
    let new_node_name = if value_a_opt.is_none() {
        &source_monkey_list[&node_name].monkey_name_a
    } else {
        &source_monkey_list[&node_name].monkey_name_b
    }
    .to_owned();

    debug_assert!(new_node_name != node_name);

    let new_value = match &source_monkey_list[&node_name].operation {
        Operand::Plus => should_equal - known,
        Operand::Minus => {
            if value_a_opt.is_none() {
                should_equal + known
            } else {
                known - should_equal
            }
        }
        Operand::Multiply => should_equal / known,
        Operand::Divide => {
            if value_a_opt.is_none() {
                should_equal * known
            } else {
                known / should_equal
            }
        }
    };
    some_solve(
        new_node_name,
        new_value,
        source_monkey_list,
        source_known_numbers,
    )
}

fn algebra_human_answer(
    source_monkey_list: &mut HashMap<String, Monkey>,
    source_known_numbers: &mut HashMap<String, i64>,
) -> i64 {
    let co_a = source_known_numbers.get(&source_monkey_list[ROOT_NAME].monkey_name_a);
    let co_b = source_known_numbers.get(&source_monkey_list[ROOT_NAME].monkey_name_b);

    let eq = co_a.unwrap_or_else(|| co_b.unwrap()).to_owned();

    let node_name = if co_a.is_none() {
        &source_monkey_list[ROOT_NAME].monkey_name_a
    } else {
        &source_monkey_list[ROOT_NAME].monkey_name_b
    }
    .to_owned();
    some_solve(node_name, eq, source_monkey_list, source_known_numbers);

    source_known_numbers[HUMAN_NAME]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            monkey_puzzle(false, "./inputs/day-21-input-test.txt").unwrap(),
            152
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            monkey_puzzle(false, "./inputs/day-21-input.txt").unwrap(),
            142707821472432
        );
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            monkey_puzzle(true, "./inputs/day-21-input-test.txt").unwrap(),
            301
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            monkey_puzzle(true, "./inputs/day-21-input.txt").unwrap(),
            3587647562851
        );
    }
}
