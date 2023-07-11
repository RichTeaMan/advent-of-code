use std::io;

use itertools::Itertools;

use crate::file_utils::read_lines;

const STARTING_ITEMS_PREFIX: &str = "Starting items: ";

const OPERATION_PREFIX: &str = "Operation: new = ";
const TEST_PREFIX: &str = "Test: divisible by ";
const TRUE_PREFIX: &str = "If true: throw to monkey ";
const FALSE_PREFIX: &str = "If false: throw to monkey ";

#[derive(Default)]
struct Monkey {
    pub items: Vec<i64>,
    pub operation: Operation,
    pub test: i64,
    pub true_result: i64,
    pub false_result: i64,
    pub inspected_items: i64,
}

impl Monkey {
    /**
     * Throws items -> (monkey id, new item)
     */
    pub fn throw_items(&mut self, worry_level: i64, mod_value: i64) -> Vec<(i64, i64)> {
        let mut results = Vec::new();

        for item in &self.items {
            self.inspected_items += 1;
            let new_value = (self.operation.calc(*item) / worry_level) % mod_value;

            let result_monkey = if new_value % self.test == 0 {
                self.true_result
            } else {
                self.false_result
            };
            results.push((result_monkey, new_value));
        }
        self.items.clear();
        results
    }
}

enum Operand {
    Plus,
    Multiply,
}

impl Operand {
    pub fn from_str(s: &str) -> Self {
        match s {
            "+" => Self::Plus,
            "*" => Self::Multiply,
            _ => panic!("Unknown operation"),
        }
    }
}

struct Operation {
    pub a: Option<i64>,

    pub operand: Operand,

    pub b: Option<i64>,
}

impl Default for Operation {
    fn default() -> Self {
        Self {
            a: Default::default(),
            operand: Operand::Plus,
            b: Default::default(),
        }
    }
}

impl Operation {
    pub fn from_str(line: &str) -> Self {
        if let Some((a_s, op_s, b_s)) = line.split(' ').collect_tuple() {
            let a = Self::value_from_str(a_s);
            let operand = Operand::from_str(op_s);
            let b = Self::value_from_str(b_s);

            Operation { a, operand, b }
        } else {
            panic!("Cannot get operation: {line}.");
        }
    }

    fn value_from_str(value: &str) -> Option<i64> {
        let mut result = None;
        if value != "old" {
            result = Some(value.parse::<i64>().unwrap());
        }
        result
    }

    pub fn calc(&self, value: i64) -> i64 {
        let av = self.a.unwrap_or(value);
        let bv = self.b.unwrap_or(value);

        match self.operand {
            Operand::Plus => av + bv,
            Operand::Multiply => av * bv,
        }
    }
}

pub fn day_11() -> io::Result<i64> {
    let positions = monkey_sim(3, 20, "./inputs/day-11-input.txt")?;
    Ok(positions)
}

pub fn day_11_part_2() -> io::Result<i64> {
    let positions = monkey_sim(1, 10000, "./inputs/day-11-input.txt")?;
    Ok(positions)
}

fn monkey_sim(worry_level: i64, rounds: i64, filename: &str) -> io::Result<i64> {
    let mut monkey_list = Vec::new();
    let mut current_monkey = Monkey::default();

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }
        let trim = line.trim();
        if trim.starts_with("Monkey") {
            if !trim.contains('0') {
                monkey_list.push(current_monkey);
            }
            current_monkey = Monkey::default();
        } else if trim.starts_with(STARTING_ITEMS_PREFIX) {
            let items_s = trim.replace(STARTING_ITEMS_PREFIX, "");
            let items = items_s
                .split(',')
                .map(|s| s.trim().parse::<i64>().unwrap())
                .collect_vec();
            current_monkey.items = items;
        } else if trim.starts_with(OPERATION_PREFIX) {
            let operation = trim.replace(OPERATION_PREFIX, "");
            current_monkey.operation = Operation::from_str(operation.as_str());
        } else if trim.starts_with(TEST_PREFIX) {
            let test_s = trim.replace(TEST_PREFIX, "");
            current_monkey.test = test_s.parse::<i64>().unwrap();
        } else if trim.starts_with(TRUE_PREFIX) {
            let value_s = trim.replace(TRUE_PREFIX, "");
            current_monkey.true_result = value_s.parse::<i64>().unwrap();
        } else if trim.starts_with(FALSE_PREFIX) {
            let value_s = trim.replace(FALSE_PREFIX, "");
            current_monkey.false_result = value_s.parse::<i64>().unwrap();
        } else {
            panic!("Unknown line {trim}.");
        }
    }

    monkey_list.push(current_monkey);

    let mut mod_value = 1;
    for monkey in &monkey_list {
        mod_value *= monkey.test;
    }

    for _ in 0..rounds {
        for monkey_i in 0..monkey_list.len() {
            let throws = monkey_list[monkey_i].throw_items(worry_level, mod_value);

            for (monkey_id, item) in throws {
                monkey_list[monkey_id as usize].items.push(item);
            }
        }
    }

    let mut item_inspected = monkey_list.iter().map(|m| m.inspected_items).collect_vec();
    item_inspected.sort();
    item_inspected.reverse();

    Ok(item_inspected[0] * item_inspected[1])
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            monkey_sim(3, 20, "./inputs/day-11-input-test.txt").unwrap(),
            10605
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            monkey_sim(3, 20, "./inputs/day-11-input.txt").unwrap(),
            108240
        );
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            monkey_sim(1, 10000, "./inputs/day-11-input-test.txt").unwrap(),
            2713310158
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            monkey_sim(1, 10000, "./inputs/day-11-input.txt").unwrap(),
            25712998901
        );
    }
}
