use itertools::Itertools;
use std::io::{self};
use utils::file_utils::read_lines;

struct Game {
    id: i32,
    hints: Vec<Hint>,
}

struct Hint {
    red: i32,
    green: i32,
    blue: i32,
}

pub fn day_2() -> io::Result<i32> {
    fetch_possible_games("./inputs/day-2-input.txt")
}
pub fn day_2_part_2() -> io::Result<i32> {
    fetch_power_of_games("./inputs/day-2-input.txt")
}

fn fetch_possible_games(filename: &str) -> io::Result<i32> {
    let games = fetch_games(filename)?;

    let mut result = 0;

    // only 12 red cubes, 13 green cubes, and 14 blue cubes

    for game in games {
        let mut possible = true;
        for hint in game.hints {
            if hint.red > 12 || hint.green > 13 || hint.blue > 14 {
                possible = false;
                break;
            }
        }
        if possible {
            result += game.id
        }
    }
    Ok(result)
}

fn fetch_power_of_games(filename: &str) -> io::Result<i32> {
    let games = fetch_games(filename)?;

    let mut result = 0;

    for game in games {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for hint in game.hints {
            red = hint.red.max(red);
            green = hint.green.max(green);
            blue = hint.blue.max(blue);
        }
        result += red * green * blue;
    }
    Ok(result)
}

fn fetch_games(filename: &str) -> io::Result<Vec<Game>> {
    let mut games = Vec::new();

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }

        if let Some((game_id_str, game_info_str)) = line.split(':').collect_tuple() {
            let game_id = sscanf::sscanf!(game_id_str, "Game {}", i32).unwrap();

            let mut hints = Vec::new();
            for game_info in game_info_str.split(';') {
                let mut hint = Hint {
                    red: 0,
                    green: 0,
                    blue: 0,
                };
                for colour in game_info.split(',') {
                    let number_str = colour
                        .chars()
                        .filter(|c| c.is_ascii_digit())
                        .collect::<String>();
                    let number = number_str.parse::<i32>().unwrap();

                    if colour.contains("red") {
                        hint.red = number;
                    } else if colour.contains("green") {
                        hint.green = number;
                    } else if colour.contains("blue") {
                        hint.blue = number;
                    } else {
                        panic!("Unknown colour. {}", colour);
                    }
                }
                hints.push(hint);
            }

            let game = Game { id: game_id, hints };
            games.push(game);
        } else {
            panic!("Unexpected parts after colon split.");
        }
    }
    Ok(games)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            fetch_possible_games("./inputs/day-2-input-test.txt").unwrap(),
            8
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            fetch_possible_games("./inputs/day-2-input.txt").unwrap(),
            2563
        );
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            fetch_power_of_games("./inputs/day-2-input-test.txt").unwrap(),
            2286
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            fetch_power_of_games("./inputs/day-2-input.txt").unwrap(),
            70768
        );
    }
}
