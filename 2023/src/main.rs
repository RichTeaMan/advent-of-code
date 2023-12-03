mod day_1;
mod day_2;
mod day_3;

use crate::day_1::{day_1, day_1_part_2};
use crate::day_2::{day_2, day_2_part_2};
use crate::day_3::{day_3, day_3_part_2};

fn main() {
    println!("Advent of Code 2023: https://adventofcode.com/2023");
    println!();

    println!("Day  1 part 1 answer: {answer}", answer = day_1().unwrap());
    println!(
        "Day  1 part 2 answer: {answer}",
        answer = day_1_part_2().unwrap()
    );
    println!("Day  2 part 1 answer: {answer}", answer = day_2().unwrap());
    println!(
        "Day  2 part 2 answer: {answer}",
        answer = day_2_part_2().unwrap()
    );
    println!("Day  3 part 1 answer: {answer}", answer = day_3().unwrap());
    println!(
        "Day  3 part 2 answer: {answer}",
        answer = day_3_part_2().unwrap()
    );
}
