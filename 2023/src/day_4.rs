use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{self},
};
use utils::file_utils::read_lines;

struct ScratchCard {
    card_id: i32,
    winning_numbers: HashSet<i32>,
    numbers: HashSet<i32>,
}

pub fn day_4() -> io::Result<i32> {
    fetch_scores("./inputs/day-4-input.txt")
}
pub fn day_4_part_2() -> io::Result<i32> {
    fetch_winning_cards_amount("./inputs/day-4-input.txt")
}

fn fetch_scratch_cards(filename: &str) -> io::Result<Vec<ScratchCard>> {
    let mut scratch_cards = Vec::new();

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }

        if let Some((card_str, numbers_str)) = line.split(':').collect_tuple() {
            let card_id_str = card_str.replace("Card ", "");
            let card_id = card_id_str.trim().parse::<i32>().unwrap();

            if let Some((winning_str, game_numbers_str)) = numbers_str.split('|').collect_tuple() {
                let winning_numbers = winning_str
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect_vec();
                let numbers = game_numbers_str
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.trim().parse::<i32>().unwrap());
                scratch_cards.push(ScratchCard {
                    card_id,
                    winning_numbers: HashSet::from_iter(winning_numbers),
                    numbers: HashSet::from_iter(numbers),
                })
            } else {
                panic!("Unexpected number format");
            }
        } else {
            panic!("Unexpected line format.")
        }
    }
    Ok(scratch_cards)
}

fn fetch_scores(filename: &str) -> io::Result<i32> {
    let cards = fetch_scratch_cards(filename)?;

    let mut score = 0;

    for card in cards {
        let count = card
            .numbers
            .iter()
            .filter(|x| card.winning_numbers.contains(x))
            .count();
        if count > 0 {
            score += 2_i32.pow(count as u32 - 1_u32);
        }
    }
    Ok(score)
}

fn fetch_winning_cards_amount(filename: &str) -> io::Result<i32> {
    let cards = fetch_scratch_cards(filename)?;
    let mut score_map = HashMap::new();
    let mut card_stack = VecDeque::new();

    // pre-calc winnings
    for card in cards {
        let count = card
            .numbers
            .iter()
            .filter(|x| card.winning_numbers.contains(x))
            .count() as i32;

        score_map.insert(card.card_id, count);
        card_stack.push_back(card.card_id);
    }

    let mut winning_cards = 0;

    while let Some(id) = card_stack.pop_front() {
        winning_cards += 1;

        let score = score_map.get(&id).unwrap();
        for n in (id + 1)..=(id + score) {
            card_stack.push_back(n);
        }
    }
    Ok(winning_cards)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(fetch_scores("./inputs/day-4-input-test.txt").unwrap(), 13);
    }

    #[test]
    fn test() {
        assert_eq!(fetch_scores("./inputs/day-4-input.txt").unwrap(), 22193);
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            fetch_winning_cards_amount("./inputs/day-4-input-test.txt").unwrap(),
            30
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            fetch_winning_cards_amount("./inputs/day-4-input.txt").unwrap(),
            5625994
        );
    }
}
