use std::io::{self};

use itertools::Itertools;

use utils::file_utils::read_lines;

pub fn day_4() -> io::Result<i32> {
    determine_winning_board("./inputs/day-4-input.txt")
}
pub fn day_4_part_2() -> io::Result<i32> {
    determine_losing_board("./inputs/day-4-input.txt")
}

struct BoardLine {
    line: Vec<i32>,
    board_id: i32,
    row: bool,
}

const BOARD_SIZE: usize = 5;

fn determine_winning_board(filename: &str) -> io::Result<i32> {
    let mut calls = Vec::new();
    let mut board_count = 0;
    let mut board_lines: Vec<BoardLine> = Vec::new();

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            let mut columns = Vec::new();
            let boards: Vec<&BoardLine> = board_lines
                .iter()
                .filter(|l| l.board_id == board_count)
                .collect_vec();

            for column_index in 0..BOARD_SIZE {
                let values = boards.iter().map(|l| l.line[column_index]).collect_vec();
                let column = BoardLine {
                    line: values,
                    board_id: board_count,
                    row: false,
                };
                columns.push(column);
            }
            board_lines.extend(columns);

            board_count += 1;
            continue;
        }

        if board_count == 0 {
            calls.extend(line.split(',').map(|v| v.parse::<i32>().unwrap()));
            continue;
        }

        let board_line = BoardLine {
            line: line
                .split(' ')
                .filter(|v| !v.is_empty())
                .map(|v| v.trim().parse::<i32>().unwrap())
                .collect(),
            board_id: board_count,
            row: true,
        };
        board_lines.push(board_line);
    }

    let mut winning_board = None;
    'call_loop: for call in calls {
        for board_line in board_lines.iter_mut() {
            let found = board_line.line.iter().position(|c| c == &call);
            if let Some(found_index) = found {
                board_line.line.remove(found_index);

                if board_line.line.is_empty() {
                    winning_board = Some((board_line.board_id, call));
                    break 'call_loop;
                }
            }
        }
    }

    if let Some((winning_board_id, call)) = winning_board {
        let board_sum = board_lines
            .iter()
            .filter(|l| l.board_id == winning_board_id && l.row)
            .map(|l| l.line.iter().sum::<i32>())
            .reduce(|acc, e| acc + e)
            .unwrap();

        return Ok(call * board_sum);
    }

    panic!("Failed to find winning board.");
}

fn determine_losing_board(filename: &str) -> io::Result<i32> {
    let mut calls = Vec::new();
    let mut board_count = 0;
    let mut board_lines: Vec<BoardLine> = Vec::new();

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            let mut columns = Vec::new();
            let boards: Vec<&BoardLine> = board_lines
                .iter()
                .filter(|l| l.board_id == board_count)
                .collect_vec();

            for column_index in 0..BOARD_SIZE {
                let values = boards.iter().map(|l| l.line[column_index]).collect_vec();
                let column = BoardLine {
                    line: values,
                    board_id: board_count,
                    row: false,
                };
                columns.push(column);
            }
            board_lines.extend(columns);

            board_count += 1;
            continue;
        }

        if board_count == 0 {
            calls.extend(line.split(',').map(|v| v.parse::<i32>().unwrap()));
            continue;
        }

        let board_line = BoardLine {
            line: line
                .split(' ')
                .filter(|v| !v.is_empty())
                .map(|v| v.trim().parse::<i32>().unwrap())
                .collect(),
            board_id: board_count,
            row: true,
        };
        board_lines.push(board_line);
    }

    let mut board_ids = board_lines
        .iter()
        .map(|l| l.board_id)
        .unique()
        .collect_vec();

    let mut losing_board = None;
    'call_loop: for call in calls {
        for board_line in board_lines.iter_mut() {
            let found = board_line.line.iter().position(|c| c == &call);
            if let Some(found_index) = found {
                board_line.line.remove(found_index);

                if board_line.line.is_empty() {
                    let found_board_id = board_ids.iter().position(|p| p == &board_line.board_id);
                    if let Some(found_board_index) = found_board_id {
                        board_ids.remove(found_board_index);
                        if board_ids.len() == 1 {
                            losing_board = Some((board_line.board_id, call));
                            break 'call_loop;
                        }
                    }
                }
            }
        }
    }

    if let Some((losing_board_id, call)) = losing_board {
        let board_sum = board_lines
            .iter()
            .filter(|l| l.board_id == losing_board_id && l.row)
            .map(|l| l.line.iter().sum::<i32>())
            .reduce(|acc, e| acc + e)
            .unwrap();

        return Ok(call * board_sum);
    }

    panic!("Failed to find losing board.");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            determine_winning_board("./inputs/day-4-input-test.txt").unwrap(),
            4512
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            determine_winning_board("./inputs/day-4-input.txt").unwrap(),
            31424
        );
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            determine_losing_board("./inputs/day-4-input-test.txt").unwrap(),
            1924
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            determine_losing_board("./inputs/day-4-input.txt").unwrap(),
            23042
        );
    }
}
