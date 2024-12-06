use itertools::Itertools;
use std::{
    collections::VecDeque,
    io::{self},
};
use utils::file_utils::read_lines;

pub fn day_5() -> io::Result<i32> {
    calc_middle_pages("./inputs/day-5-input.txt")
}
pub fn day_5_part_2() -> io::Result<i32> {
    calc_and_fix_middle_pages("./inputs/day-5-input.txt")
}

fn calc_middle_pages(filename: &str) -> io::Result<i32> {
    let (ordering_rules, update_list) = fetch_rules(filename)?;

    let mut middle_cumm = 0;
    for update in update_list {
        let mut invalid_pages = vec![];
        let mut valid = true;
        for page in &update {
            if invalid_pages.contains(page) {
                valid = false;
                break;
            }
            for (order_page, after_page) in &ordering_rules {
                if page == after_page {
                    invalid_pages.push(*order_page);
                }
            }
        }
        if valid {
            middle_cumm += update[update.len() / 2];
        }
    }

    Ok(middle_cumm)
}

fn calc_and_fix_middle_pages(filename: &str) -> io::Result<i32> {
    let (ordering_rules, update_list) = fetch_rules(filename)?;

    let mut update_queue =
        VecDeque::from(update_list.iter().map(|x| (x.clone(), false)).collect_vec());

    let mut middle_cumm = 0;

    while let Some((mut update, corrected)) = update_queue.pop_front() {
        let mut invalid_pages = vec![];
        let mut valid = true;
        let middle = update[update.len() / 2];
        for (i, page) in update.iter().enumerate() {
            if invalid_pages.contains(page) {
                update.swap(i - 1, i);
                update_queue.push_front((update, true));
                valid = false;
                break;
            }
            for (order_page, after_page) in &ordering_rules {
                if page == after_page {
                    invalid_pages.push(*order_page);
                }
            }
        }
        if valid && corrected {
            middle_cumm += middle;
        }
    }

    Ok(middle_cumm)
}

fn fetch_rules(filename: &str) -> io::Result<(Vec<(i32, i32)>, Vec<Vec<i32>>)> {
    let lines = read_lines(filename)?;
    let mut order_rules = vec![];
    let mut update_list = vec![];

    for line in lines.flatten() {
        if line.contains("|") {
            let order_rule = sscanf::sscanf!(line, "{}|{}", i32, i32).unwrap();
            order_rules.push(order_rule);
        } else if line.contains(",") {
            let update = line
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect_vec();
            update_list.push(update);
        }
    }
    Ok((order_rules, update_list))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            calc_middle_pages("./inputs/day-5-input-test.txt").unwrap(),
            143
        );
    }

    #[test]
    fn test() {
        assert_eq!(calc_middle_pages("./inputs/day-5-input.txt").unwrap(), 4135);
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            calc_and_fix_middle_pages("./inputs/day-5-input-test.txt").unwrap(),
            123
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            calc_and_fix_middle_pages("./inputs/day-5-input.txt").unwrap(),
            5285
        );
    }
}
