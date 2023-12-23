use std::{collections::HashMap, io};


use utils::file_utils::read_lines;

pub fn day_21() -> io::Result<i32> {
    eval_game_from_file("./inputs/day-21-input.txt")
}
pub fn day_21_part_2() -> io::Result<u128> {
    eval_dirac_game_from_file("./inputs/day-21-input.txt")
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
        self.last_roll %= 100;
        self.roll_count += 1;
        self.last_roll
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Player {
    score: i32,
    position: i32,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct GameState {
    player_1: Player,
    player_2: Player,
}

impl GameState {
    fn from_positions(player_1_position: i32, player_2_position: i32) -> Self {
        GameState {
            player_1: Player {
                position: player_1_position,
                score: 0,
            },
            player_2: Player {
                position: player_2_position,
                score: 0,
            },
        }
    }
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

    while players.iter().map(|p| p.score).max().unwrap() < winning_score {
        // roll three times
        let roll = dice.roll() + dice.roll() + dice.roll();

        let player = players.get_mut(player_index).unwrap();
        let new_pos = ((roll + (player.position - 1)) % 10) + 1;
        debug_assert!((1..=10).contains(&new_pos));

        player.position = new_pos;
        player.score += new_pos;

        player_index += 1;
        player_index %= player_count;
    }
    players.iter().map(|p| p.score).min().unwrap() * dice.roll_count
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

fn eval_dirac_game_from_file(filename: &str) -> io::Result<u128> {
    let players = load_players(filename)?;
    if players.len() != 2 {
        panic!(
            "Irregular number of players: {}. Expected 2.",
            players.len()
        );
    }

    let player_1_position = players.first().unwrap().position;
    let player_2_position = players.get(1).unwrap().position;

    Ok(eval_dirac_game(player_1_position, player_2_position))
}

fn eval_dirac_game(player_1_position: i32, player_2_position: i32) -> u128 {
    let mut combos: HashMap<i32, u128> = HashMap::new();
    // pre calc all the combos of 3 dice
    for a in 1..=3 {
        for b in 1..=3 {
            for c in 1..=3 {
                let total = a + b + c;
                combos.entry(total).and_modify(|e| *e += 1).or_insert(1);
            }
        }
    }

    let mut game_states = HashMap::new();
    game_states.insert(
        GameState::from_positions(player_1_position, player_2_position),
        1_u128,
    );

    let mut p_1_wins = 0_u128;
    let mut p_2_wins = 0_u128;

    while !game_states.is_empty() {
        // process player 1
        let mut new_game_states = HashMap::new();
        for (state, count) in &game_states {
            for (roll, roll_count) in &combos {
                let mut new_state = *state;

                let new_pos = ((roll + (state.player_1.position - 1)) % 10) + 1;
                debug_assert!((1..=10).contains(&new_pos));

                new_state.player_1.position = new_pos;
                new_state.player_1.score += new_pos;
                let instances: u128 = count * roll_count;

                if new_state.player_1.score >= 21 {
                    p_1_wins += instances;
                } else {
                    new_game_states
                        .entry(new_state)
                        .and_modify(|e| *e += instances)
                        .or_insert(instances);
                }
            }
        }
        game_states = new_game_states;

        // process player 2
        let mut new_game_states = HashMap::new();
        for (state, count) in &game_states {
            for (roll, roll_count) in &combos {
                let mut new_state = *state;

                let new_pos = ((roll + (state.player_2.position - 1)) % 10) + 1;
                debug_assert!((1..=10).contains(&new_pos));

                new_state.player_2.position = new_pos;
                new_state.player_2.score += new_pos;
                let instances: u128 = count * roll_count;

                if new_state.player_2.score >= 21 {
                    p_2_wins += instances;
                } else {
                    new_game_states
                        .entry(new_state)
                        .and_modify(|e| *e += instances)
                        .or_insert(instances);
                }
            }
        }
        game_states = new_game_states;
    }

    p_1_wins.max(p_2_wins)
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

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            eval_dirac_game_from_file("./inputs/day-21-input-test.txt").unwrap(),
            444356092776315_u128
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            eval_dirac_game_from_file("./inputs/day-21-input.txt").unwrap(),
            49975322685009_u128
        );
    }
}
