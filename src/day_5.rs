use std::{
    collections::VecDeque,
    io::{self},
};

use itertools::Itertools;

use crate::file_utils::read_lines;

struct CraneStacks {
    pub stacks: Vec<VecDeque<char>>,
}

impl CraneStacks {
    fn read_top_letters(&self) -> String {
        let mut result = "".to_string();
        for stack in &self.stacks {
            result += &stack[0].to_string();
        }
        result
    }
}

pub fn day_5() -> io::Result<String> {
    let path = "./day-5-input.txt";
    let mut crane_stacks = fetch_starting_stacks(path)?;
    crane_stacks = execute_crane_instructions(path, crane_stacks)?;

    Ok(crane_stacks.read_top_letters())
}

pub fn day_5_part_2() -> io::Result<String> {
    let path = "./day-5-input.txt";
    let mut crane_stacks = fetch_starting_stacks(path)?;
    crane_stacks = execute_bulk_crane_instructions(path, crane_stacks)?;

    Ok(crane_stacks.read_top_letters())
}

fn fetch_starting_stacks(filepath: &str) -> io::Result<CraneStacks> {
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    let lines = read_lines(filepath)?;
    for line in lines.flatten() {
        if !line.contains('[') {
            break;
        }

        for (current_stack, stack_ele) in line
            .chars()
            .chunks(4)
            .into_iter()
            .map(|chunk| chunk.collect::<String>())
            .collect::<Vec<String>>()
            .into_iter()
            .enumerate()
        {
            if stacks.len() <= current_stack {
                stacks.push(VecDeque::new());
            }

            if stack_ele.trim().is_empty() {
                continue;
            }
            let trim: &[_] = &['[', ']', ' '];
            let trimmed = stack_ele.trim_matches(trim).to_string();

            assert!(!trimmed.is_empty());
            let stack_opt = stacks.get_mut(current_stack);
            stack_opt
                .unwrap()
                .push_back(trimmed.chars().next().unwrap());
        }
    }
    Ok(CraneStacks { stacks })
}

fn execute_crane_instructions(
    filepath: &str,
    mut crane_stacks: CraneStacks,
) -> io::Result<CraneStacks> {
    let lines = read_lines(filepath)?;
    for line in lines.flatten() {
        if !line.contains("move") {
            continue;
        }

        // eg move 1 from 2 to 1
        if let Some((_move, crate_count_s, _from, start_stack_id_s, _to, end_stack_id_s)) =
            line.split_whitespace().collect_tuple()
        {
            let start_stack_id: usize = str::parse::<usize>(start_stack_id_s).unwrap() - 1;
            let end_stack_id: usize = str::parse::<usize>(end_stack_id_s).unwrap() - 1;
            let crate_count: i32 = str::parse(crate_count_s).unwrap();

            for _ in 0..crate_count {
                let crane_crate = crane_stacks
                    .stacks
                    .get_mut(start_stack_id)
                    .unwrap()
                    .pop_front()
                    .unwrap();
                crane_stacks
                    .stacks
                    .get_mut(end_stack_id)
                    .unwrap()
                    .push_front(crane_crate);
            }
        } else {
            panic!("Unexpected string format: {line}");
        }
    }
    Ok(crane_stacks)
}

fn execute_bulk_crane_instructions(
    filepath: &str,
    mut crane_stacks: CraneStacks,
) -> io::Result<CraneStacks> {
    let lines = read_lines(filepath)?;
    for line in lines.flatten() {
        if !line.contains("move") {
            continue;
        }

        // eg move 1 from 2 to 1
        if let Some((_move, crate_count_s, _from, start_stack_id_s, _to, end_stack_id_s)) =
            line.split_whitespace().collect_tuple()
        {
            let start_stack_id: usize = str::parse::<usize>(start_stack_id_s).unwrap() - 1;
            let end_stack_id: usize = str::parse::<usize>(end_stack_id_s).unwrap() - 1;
            let crate_count: i32 = str::parse(crate_count_s).unwrap();

            let mut temp_stack = Vec::new();
            for _ in 0..crate_count {
                let crane_crate = crane_stacks
                    .stacks
                    .get_mut(start_stack_id)
                    .unwrap()
                    .pop_front()
                    .unwrap();

                temp_stack.push(crane_crate);
            }
            temp_stack.reverse();
            for crane_crate in temp_stack {
                crane_stacks
                    .stacks
                    .get_mut(end_stack_id)
                    .unwrap()
                    .push_front(crane_crate);
            }
        } else {
            panic!("Unexpected string format: {line}");
        }
    }
    Ok(crane_stacks)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn fetch_starting_stacks_small_test() {
        assert_eq!(
            fetch_starting_stacks("./day-5-input-test.txt")
                .unwrap()
                .read_top_letters(),
            "NDP"
        );
    }

    #[test]
    fn execute_crane_instructions_small_test() {
        let mut crane_stacks = fetch_starting_stacks("./day-5-input-test.txt").unwrap();
        crane_stacks = execute_crane_instructions("./day-5-input-test.txt", crane_stacks).unwrap();
        assert_eq!(crane_stacks.read_top_letters(), "CMZ");
    }

    #[test]
    fn execute_crane_instructions_test() {
        let mut crane_stacks = fetch_starting_stacks("./day-5-input.txt").unwrap();
        crane_stacks = execute_crane_instructions("./day-5-input.txt", crane_stacks).unwrap();
        assert_eq!(crane_stacks.read_top_letters(), "FRDSQRRCD");
    }

    #[test]
    fn execute_bulk_crane_instructions_small_test() {
        let mut crane_stacks = fetch_starting_stacks("./day-5-input-test.txt").unwrap();
        crane_stacks =
            execute_bulk_crane_instructions("./day-5-input-test.txt", crane_stacks).unwrap();
        assert_eq!(crane_stacks.read_top_letters(), "MCD");
    }

    #[test]
    fn execute_bulk_crane_instructions_test() {
        let mut crane_stacks = fetch_starting_stacks("./day-5-input.txt").unwrap();
        crane_stacks = execute_bulk_crane_instructions("./day-5-input.txt", crane_stacks).unwrap();
        assert_eq!(crane_stacks.read_top_letters(), "HRFTQVWNN");
    }
}
