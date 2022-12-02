use std::{
    io::{self},
    path::Path,
};

use crate::file_utils::read_lines;

#[derive(Eq, PartialEq)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn player(line: &str) -> Self {
        if line.contains("A") {
            return Self::Rock;
        } else if line.contains("B") {
            return Self::Paper;
        } else if line.contains("C") {
            return Self::Scissors;
        }
        panic!("No player symbol.");
    }

    pub fn opponent(line: &str) -> Self {
        if line.contains("X") {
            return Self::Rock;
        } else if line.contains("Y") {
            return Self::Paper;
        } else if line.contains("Z") {
            return Self::Scissors;
        }
        panic!("No opponent symbol.");
    }

    pub fn score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

pub fn day_2() -> io::Result<i32> {
    game_file_loader("./day-2-input.txt")
}

pub fn game_file_loader<P>(filename: P) -> io::Result<i32>
where
    P: AsRef<Path>,
{
    let mut score = 0;
    let lines = read_lines(filename)?;
    for line_res in lines {
        let line = line_res?;

        if line.is_empty() {
            continue;
        }

        let opponent = Shape::player(&line);
        let player = Shape::opponent(&line);

        score += player.score();

        if player == opponent {
            score += 3;
        } else {
            score += match player {
                Shape::Rock => {
                    if opponent == Shape::Scissors {
                        6
                    } else {
                        0
                    }
                }
                Shape::Paper => {
                    if opponent == Shape::Rock {
                        6
                    } else {
                        0
                    }
                }
                Shape::Scissors => {
                    if opponent == Shape::Paper {
                        6
                    } else {
                        0
                    }
                }
            }
        }
    }
    Ok(score)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(game_file_loader("./day-2-input-test.txt").unwrap(), 15);
    }

    #[test]
    fn test() {
        assert_eq!(day_2().unwrap(), 17189);
    }
}
