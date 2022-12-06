mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod file_utils;

use crate::day_1::{day_1, day_1_part_2};
use crate::day_2::{day_2, day_2_part_2};
use crate::day_3::{day_3, day_3_part_2};
use crate::day_4::{day_4, day_4_part_2};
use crate::day_5::{day_5, day_5_part_2};
use crate::day_6::{day_6, day_6_part_2};

fn main() {
    println!("Day 1 part 1 answer: {answer}", answer = day_1().unwrap());
    println!(
        "Day 1 part 2 answer: {answer}",
        answer = day_1_part_2().unwrap()
    );
    println!("Day 2 part 1 answer: {answer}", answer = day_2().unwrap());
    println!(
        "Day 2 part 2 answer: {answer}",
        answer = day_2_part_2().unwrap()
    );
    println!("Day 3 part 1 answer: {answer}", answer = day_3().unwrap());
    println!(
        "Day 3 part 2 answer: {answer}",
        answer = day_3_part_2().unwrap()
    );
    println!("Day 4 part 1 answer: {answer}", answer = day_4().unwrap());
    println!(
        "Day 4 part 2 answer: {answer}",
        answer = day_4_part_2().unwrap()
    );
    println!("Day 5 part 1 answer: {answer}", answer = day_5().unwrap());
    println!(
        "Day 5 part 2 answer: {answer}",
        answer = day_5_part_2().unwrap()
    );
    println!("Day 6 part 1 answer: {answer}", answer = day_6().unwrap());
}
