use std::{
    collections::HashMap,
    io::{self},
};

use itertools::Itertools;

use crate::file_utils::read_lines;

const RESET_TIMER: i32 = 6;
const NEW_TIMER: i32 = 8;

pub fn day_6() -> io::Result<i64> {
    process_generations(80, "./inputs/day-6-input.txt")
}
pub fn day_6_part_2() -> io::Result<i64> {
    process_generations(256, "./inputs/day-6-input.txt")
}

struct Generation {
    timer: i32,
    amount: i64,
}

fn load_generation(filename: &str) -> io::Result<Vec<Generation>> {
    let mut generations: HashMap<i32, Generation> = HashMap::new();

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }

        for split in line.split(',') {
            let timer = split.parse::<i32>().unwrap();

            if let Some(generation) = generations.get_mut(&timer) {
                generation.amount += 1;
            } else {
                generations.insert(timer, Generation { timer, amount: 1 });
            }
        }
    }
    Ok(generations.into_iter().map(|e| e.1).collect_vec())
}

fn process_generation(generations: Vec<Generation>) -> Vec<Generation> {
    let mut new_fish = 0;
    let mut new_gens: Vec<Generation> = Vec::new();
    for mut gen in generations {
        if gen.timer == 0 {
            new_fish += gen.amount;
            gen.timer = RESET_TIMER;
        } else {
            gen.timer -= 1;
            new_gens.push(gen);
        }
    }

    if let Ok(reset_gen) = new_gens
        .iter_mut()
        .filter(|generation| generation.timer == RESET_TIMER)
        .exactly_one()
    {
        reset_gen.amount += new_fish;
    } else {
        new_gens.push(Generation {
            timer: RESET_TIMER,
            amount: new_fish,
        });
    }
    new_gens.push(Generation {
        timer: NEW_TIMER,
        amount: new_fish,
    });

    new_gens
}

fn process_generations(days: i32, filename: &str) -> io::Result<i64> {
    let mut gen = load_generation(filename)?;
    for _ in 0..days {
        gen = process_generation(gen);
    }
    Ok(gen.iter().map(|gen| gen.amount).sum::<i64>())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            process_generations(80, "./inputs/day-6-input-test.txt").unwrap(),
            5934
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            process_generations(80, "./inputs/day-6-input.txt").unwrap(),
            362639
        );
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            process_generations(256, "./inputs/day-6-input-test.txt").unwrap(),
            26_984_457_539
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            process_generations(256, "./inputs/day-6-input.txt").unwrap(),
            1_639_854_996_917
        );
    }
}
