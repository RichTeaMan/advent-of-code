mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

use crate::day_1::{day_1, day_1_part_2};
use crate::day_10::{day_10, day_10_part_2};
use crate::day_11::{day_11, day_11_part_2};
use crate::day_12::{day_12, day_12_part_2};
use crate::day_13::{day_13, day_13_part_2};
use crate::day_14::{day_14, day_14_part_2};
use crate::day_15::{day_15, day_15_part_2};
use crate::day_2::{day_2, day_2_part_2};
use crate::day_3::{day_3, day_3_part_2};
use crate::day_4::{day_4, day_4_part_2};
use crate::day_5::{day_5, day_5_part_2};
use crate::day_6::{day_6, day_6_part_2};
use crate::day_7::{day_7, day_7_part_2};
use crate::day_8::{day_8, day_8_part_2};
use crate::day_9::{day_9, day_9_part_2};

fn main() {
    println!("Advent of Code 2021: https://adventofcode.com/2021");
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
    println!("Day  4 part 1 answer: {answer}", answer = day_4().unwrap());
    println!(
        "Day  4 part 2 answer: {answer}",
        answer = day_4_part_2().unwrap()
    );
    println!("Day  5 part 1 answer: {answer}", answer = day_5().unwrap());
    println!(
        "Day  5 part 2 answer: {answer}",
        answer = day_5_part_2().unwrap()
    );
    println!("Day  6 part 1 answer: {answer}", answer = day_6().unwrap());
    println!(
        "Day  6 part 2 answer: {answer}",
        answer = day_6_part_2().unwrap()
    );
    println!("Day  7 part 1 answer: {answer}", answer = day_7().unwrap());
    println!(
        "Day  7 part 2 answer: {answer}",
        answer = day_7_part_2().unwrap()
    );
    println!("Day  8 part 1 answer: {answer}", answer = day_8().unwrap());
    println!(
        "Day  8 part 2 answer: {answer}",
        answer = day_8_part_2().unwrap()
    );
    println!("Day  9 part 1 answer: {answer}", answer = day_9().unwrap());
    println!(
        "Day  9 part 2 answer: {answer}",
        answer = day_9_part_2().unwrap()
    );
    println!("Day 10 part 1 answer: {answer}", answer = day_10().unwrap());
    println!(
        "Day 10 part 2 answer: {answer}",
        answer = day_10_part_2().unwrap()
    );
    println!("Day 11 part 1 answer: {answer}", answer = day_11().unwrap());
    println!(
        "Day 11 part 2 answer: {answer}",
        answer = day_11_part_2().unwrap()
    );
    println!("Day 12 part 1 answer: {answer}", answer = day_12().unwrap());
    println!(
        "Day 12 part 2 answer: {answer}",
        answer = day_12_part_2().unwrap()
    );
    println!("Day 13 part 1 answer: {answer}", answer = day_13().unwrap());
    println!(
        "Day 13 part 2 answer:\n{answer}",
        answer = day_13_part_2().unwrap()
    );
    println!("Day 14 part 1 answer: {answer}", answer = day_14().unwrap());
    println!(
        "Day 14 part 2 answer: {answer}",
        answer = day_14_part_2().unwrap()
    );
    println!("Day 15 part 1 answer: {answer}", answer = day_15().unwrap());
    println!(
        "Day 15 part 2 answer: {answer}",
        answer = day_15_part_2().unwrap()
    );
}
