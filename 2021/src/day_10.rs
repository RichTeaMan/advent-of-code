use std::{
    collections::VecDeque,
    io::{self},
};

use crate::file_utils::read_lines;

pub fn day_10() -> io::Result<i32> {
    find_syntax_errors("./inputs/day-10-input.txt")
}
pub fn day_10_part_2() -> io::Result<i64> {
    solve_syntax_errors("./inputs/day-10-input.txt")
}

fn char_is_open(c: &char) -> bool {
    match c {
        '(' | '{' | '<' | '[' => true,
        ')' | '}' | '>' | ']' => false,
        _ => panic!("Unknown character: {c}"),
    }
}

fn char_get_closed(c: &char) -> char {
    match c {
        '(' => ')',
        '{' => '}',
        '[' => ']',
        '<' => '>',
        _ => panic!("Unknown character: {c}"),
    }
}

fn brace_match(open: char, close: char) -> bool {
    open == '(' && close == ')'
        || open == '{' && close == '}'
        || open == '[' && close == ']'
        || open == '<' && close == '>'
}

fn brace_score(c: &char) -> i32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Unknown character: {c}"),
    }
}

fn solve_brace_score(c: &char) -> i32 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Unknown character: {c}"),
    }
}

fn find_syntax_errors(filename: &str) -> io::Result<i32> {
    let mut score = 0;

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }

        let mut stack = VecDeque::new();

        for c in line.trim().chars() {
            if c.is_whitespace() {
                continue;
            }
            if char_is_open(&c) {
                stack.push_front(c);
            } else {
                if let Some(current) = stack.pop_front() {
                    if brace_match(current, c) {
                        continue;
                    }
                }
                score += brace_score(&c);
                break;
            }
        }
    }
    Ok(score)
}

fn solve_syntax_errors(filename: &str) -> io::Result<i64> {
    let mut scores = Vec::new();

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }

        let mut stack = VecDeque::new();

        let mut discard = false;
        for c in line.trim().chars() {
            if c.is_whitespace() {
                continue;
            }
            if char_is_open(&c) {
                stack.push_front(c);
            } else {
                if let Some(current) = stack.pop_front() {
                    if brace_match(current, c) {
                        continue;
                    }
                }
                discard = true;
                break;
            }
        }
        if discard {
            continue;
        }

        let mut score: i64 = 0;
        while let Some(open) = stack.pop_front() {
            let close = char_get_closed(&open);
            score *= 5;
            score += solve_brace_score(&close) as i64;
        }
        scores.push(score);
    }

    scores.sort();
    let score = scores.get(scores.len() / 2).unwrap().to_owned();

    Ok(score)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            find_syntax_errors("./inputs/day-10-input-test.txt").unwrap(),
            26397
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            find_syntax_errors("./inputs/day-10-input.txt").unwrap(),
            366027
        );
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            solve_syntax_errors("./inputs/day-10-input-test.txt").unwrap(),
            288957
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            solve_syntax_errors("./inputs/day-10-input.txt").unwrap(),
            1118645287
        );
    }
}
