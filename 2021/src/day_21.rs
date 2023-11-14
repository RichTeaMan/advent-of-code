use std::io;

use utils::file_utils::read_lines;

pub fn day_21() -> io::Result<i32> {
    eval_game_from_file("./inputs/day-21-input.txt")
}
pub fn day_21_part_2() -> io::Result<i64> {
    panic!() //find_largest_magnitude_from_file("./inputs/day-21-input.txt")
}

#[derive(Debug)]
struct DeterministicDice {
    last_roll: i32,
    roll_count: i32,
}

impl DeterministicDice {
    fn new() -> Self {
        DeterministicDice {
            last_roll: 100,
            roll_count: 0,
        }
    }

    fn roll(&mut self) -> i32 {
        self.last_roll += 1;
        self.last_roll = self.last_roll % 100;
        self.roll_count += 1;
        self.last_roll
    }
}

#[derive(Debug)]
struct Player {
    score: i32,
    position: i32,
}

fn eval_game_from_file(filename: &str) -> io::Result<i32> {
    let players = load_players(filename)?;
    Ok(eval_game(players))
}

fn eval_game(mut players: Vec<Player>) -> i32 {
    let mut dice = DeterministicDice::new();

    let player_count = players.len();

    let mut player_index = 0;

    let winning_score = 1000;

    while &players.iter().map(|p| p.score).max().unwrap() < &winning_score {
        // roll three times
        let roll = dice.roll() + dice.roll() + dice.roll();

        let mut player = players.get_mut(player_index).unwrap();
        //.position;
        let new_pos = ((roll + (player.position - 1)) % 10) + 1;
        debug_assert!(new_pos >= 1 && new_pos <= 10);

        player.position = new_pos;
        player.score += new_pos;

        player_index += 1;
        player_index %= player_count;
    }
    &players.iter().map(|p| p.score).min().unwrap() * dice.roll_count
}

fn load_players(filename: &str) -> io::Result<Vec<Player>> {
    let mut players = Vec::new();
    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }

        let (_, position) =
            sscanf::sscanf!(line, "Player {} starting position: {}", i32, i32).unwrap();
        players.push(Player { position, score: 0 });
    }
    Ok(players)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            eval_game_from_file("./inputs/day-21-input-test.txt").unwrap(),
            739785
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            eval_game_from_file("./inputs/day-21-input.txt").unwrap(),
            571032
        );
    }

    //  #[test]
    //  fn part_2_small_test() {
    //      assert_eq!(
    //          find_largest_magnitude_from_file("./inputs/day-18-input-test.txt").unwrap(),
    //          3993
    //      );
    //  }
    //
    //  #[test]
    //  fn part_2_test() {
    //      assert_eq!(
    //          find_largest_magnitude_from_file("./inputs/day-18-input.txt").unwrap(),
    //          4673
    //      );
    //  }
}
