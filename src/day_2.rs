use std::{
    io::{self},
    path::Path,
};

use crate::file_utils::read_lines;

#[derive(Eq, PartialEq, Clone)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn opponent(line: &str) -> Self {
        if line.contains('A') {
            return Self::Rock;
        } else if line.contains('B') {
            return Self::Paper;
        } else if line.contains('C') {
            return Self::Scissors;
        }
        panic!("No player symbol.");
    }

    pub fn player(line: &str) -> Self {
        if line.contains('X') {
            return Self::Rock;
        } else if line.contains('Y') {
            return Self::Paper;
        } else if line.contains('Z') {
            return Self::Scissors;
        }
        panic!("No opponent symbol.");
    }

    pub fn from_result(line: &str, opponent: &Self) -> Self {
        // X means you need to lose
        // Y means you need to end the round in a draw
        // Z means you need to win. Good luck!"

        // lose
        if line.contains('X') {
            return opponent.beats();
        }
        // draw
        else if line.contains('Y') {
            return opponent.clone();
        }
        // wins
        else if line.contains('Z') {
            return opponent.loses();
        }
        panic!("Unknown game result.");
    }

    pub fn score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    /**
     * Gets the shape this shape beats.
     */
    pub fn beats(&self) -> Self {
        match self {
            Shape::Rock => Self::Scissors,
            Shape::Paper => Self::Rock,
            Shape::Scissors => Self::Paper,
        }
    }

    /**
     * Gets the shape this shape loses to.
     */
    pub fn loses(&self) -> Self {
        match self {
            Shape::Rock => Self::Paper,
            Shape::Paper => Self::Scissors,
            Shape::Scissors => Self::Rock,
        }
    }
}

pub fn day_2() -> io::Result<i32> {
    game_file_loader("./day-2-input.txt")
}

pub fn day_2_part_2() -> io::Result<i32> {
    part_2_game_file_loader("./day-2-input.txt")
}

fn game_file_loader<P>(filename: P) -> io::Result<i32>
where
    P: AsRef<Path>,
{
    let mut score = 0;
    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }

        let opponent = Shape::opponent(&line);
        let player = Shape::player(&line);

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

fn part_2_game_file_loader<P>(filename: P) -> io::Result<i32>
where
    P: AsRef<Path>,
{
    let mut score = 0;
    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }

        let opponent = Shape::opponent(&line);

        let player = Shape::from_result(&line, &opponent);

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
    fn part_2_small_test() {
        assert_eq!(
            part_2_game_file_loader("./day-2-input-test.txt").unwrap(),
            12
        );
    }

    #[test]
    fn test() {
        assert_eq!(day_2().unwrap(), 17189);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(day_2_part_2().unwrap(), 13490);
    }
}
