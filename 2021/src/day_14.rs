use std::{collections::HashMap, io};

use itertools::Itertools;

use crate::file_utils::read_lines;

pub fn day_14() -> io::Result<i64> {
    count_polymer_parts(10, "./inputs/day-14-input.txt")
}
pub fn day_14_part_2() -> io::Result<i64> {
    count_polymer_parts(40, "./inputs/day-14-input.txt")
}

struct Polymer {
    template: Vec<char>,
    insertions: HashMap<String, char>,
}

fn load_polymers(filename: &str) -> io::Result<Polymer> {
    let mut insertions = HashMap::new();
    let mut template = Vec::new();

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }

        if line.contains('>') {
            let (pattern, insertion) = sscanf::sscanf!(line, "{} -> {}", String, char).unwrap();
            insertions.insert(pattern, insertion);
        } else {
            template = line.chars().collect_vec();
        }
    }
    Ok(Polymer {
        template,
        insertions,
    })
}

fn count_polymer_parts(steps: i32, filename: &str) -> io::Result<i64> {
    let polymer = load_polymers(filename)?;

    let mut pairs = HashMap::new();
    for i in 0..polymer.template.len() - 1 {
        let pair = polymer.template.get(i).unwrap().to_string()
            + &polymer.template.get(i + 1).unwrap().to_string();

        pairs.entry(pair).and_modify(|x| *x += 1).or_insert(1_i64);
    }

    for _ in 0..steps {
        let mut new_pairs = HashMap::new();
        for (pair, freq) in pairs {
            if let Some(insertion) = polymer.insertions.get(&pair) {
                let mut chars = pair.chars();
                let a = chars.next().unwrap();
                let b = chars.next().unwrap();
                let pair_a = format!("{a}{insertion}");
                let pair_b = format!("{insertion}{b}");
                new_pairs
                    .entry(pair_a)
                    .and_modify(|x| *x += freq)
                    .or_insert(freq);
                new_pairs
                    .entry(pair_b)
                    .and_modify(|x| *x += freq)
                    .or_insert(freq);
            } else {
                new_pairs.insert(pair, freq);
            }
        }
        pairs = new_pairs;
    }

    let mut counts = HashMap::new();
    counts.insert(polymer.template.last().unwrap().to_owned(), 1_i64);
    for (pair, freq) in pairs {
        counts
            .entry(pair.chars().next().unwrap())
            .and_modify(|x| *x += freq)
            .or_insert(freq);
    }

    let min = counts.values().min().unwrap();
    let max = counts.values().max().unwrap();
    Ok(max - min)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            count_polymer_parts(10, "./inputs/day-14-input-test.txt").unwrap(),
            1588
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            count_polymer_parts(10, "./inputs/day-14-input.txt").unwrap(),
            2587
        );
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            count_polymer_parts(40, "./inputs/day-14-input-test.txt").unwrap(),
            2188189693529
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            count_polymer_parts(40, "./inputs/day-14-input.txt").unwrap(),
            3318837563123
        );
    }
}
