use itertools::Itertools;
use std::{
    collections::HashSet,
    io::{self},
};
use utils::file_utils::read_lines;

pub fn day_7() -> io::Result<i64> {
    calc_score("./inputs/day-7-input.txt")
}
pub fn day_7_part_2() -> io::Result<i64> {
    calc_score_with_concat("./inputs/day-7-input.txt")
}

fn calc_score(line: &str) -> io::Result<i64> {
    let calibrations = fetch_digits(line).unwrap();

    let mut score = 0;

    for (answer, inputs) in calibrations {
        let mut attempts = HashSet::new();
        attempts.insert(inputs[0]);
        for input in inputs.iter().skip(1) {
            let mut new_attempts = HashSet::new();
            for a in attempts {
                let add = a + input;
                let mult = a * input;

                if add <= answer {
                    new_attempts.insert(add);
                }
                if mult <= answer {
                    new_attempts.insert(mult);
                }
            }
            attempts = new_attempts;
        }
        for a in attempts {
            if a == answer {
                score += answer;
                break;
            }
        }
    }

    Ok(score)
}

fn calc_score_with_concat(line: &str) -> io::Result<i64> {
    let calibrations = fetch_digits(line).unwrap();

    let mut score = 0;

    for (answer, inputs) in calibrations {
        let mut attempts = HashSet::new();
        attempts.insert(inputs[0]);
        for input in inputs.iter().skip(1) {
            let mut new_attempts = HashSet::new();
            for a in attempts {
                let add = a + input;
                let mult = a * input;
                let concat = format!("{}{}", a, input).parse::<i64>().unwrap(); // i know.

                if add <= answer {
                    new_attempts.insert(add);
                }
                if mult <= answer {
                    new_attempts.insert(mult);
                }
                if concat <= answer {
                    new_attempts.insert(concat);
                }
            }
            attempts = new_attempts;
        }
        for a in attempts {
            if a == answer {
                score += answer;
                break;
            }
        }
    }

    Ok(score)
}

fn fetch_digits(filename: &str) -> io::Result<Vec<(i64, Vec<i64>)>> {
    let lines = read_lines(filename)?;
    let mut elements = vec![];
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }
        if let Some((answer_str, input_str)) =
            line.split(':').filter(|&s| !s.is_empty()).collect_tuple()
        {
            let answer = answer_str.parse::<i64>().unwrap();
            let inputs = input_str
                .split(' ')
                .filter(|&s| !s.is_empty())
                .map(|x| x.parse::<i64>().unwrap())
                .collect_vec();
            elements.push((answer, inputs));
        }
    }
    Ok(elements)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(calc_score("./inputs/day-7-input-test.txt").unwrap(), 3749);
    }

    #[test]
    fn test() {
        assert_eq!(
            calc_score("./inputs/day-7-input.txt").unwrap(),
            303766880536
        );
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            calc_score_with_concat("./inputs/day-7-input-test.txt").unwrap(),
            11387
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            calc_score_with_concat("./inputs/day-7-input.txt").unwrap(),
            337041851384440
        );
    }
}
