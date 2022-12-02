mod day_1;
mod day_2;
mod file_utils;

use crate::day_2::{day_2, day_2_part_2};

fn main() {
    let answer = day_2_part_2().unwrap();
    println!("Answer: {answer}");
}
