use itertools::Itertools;
use std::{
    collections::VecDeque,
    io::{self},
};
use utils::file_utils::read_lines;

pub fn day_9() -> io::Result<i64> {
    calc_checksum("./inputs/day-9-input.txt")
}
pub fn day_9_part_2() -> io::Result<i64> {
    calc_whole_file_checksum("./inputs/day-9-input.txt")
}

fn calc_checksum(filename: &str) -> io::Result<i64> {
    let file_blocks = fetch_file_blocks(filename)?;
    let mut file_blocks = VecDeque::from(file_blocks);

    let mut defragged_blocks = vec![];
    while let Some(file_block) = file_blocks.pop_front() {
        if file_block.free_space == 0 {
            defragged_blocks.push(file_block);
            continue;
        } else {
            defragged_blocks.push(FileBlock {
                id: file_block.id,
                file_length: file_block.file_length,
                free_space: 0,
            });
        }

        let mut free_space = file_block.free_space;

        while free_space > 0 {
            if let Some(end_file_block) = file_blocks.pop_back() {
                if end_file_block.file_length >= free_space {
                    let fragged_end_block = FileBlock {
                        id: end_file_block.id,
                        file_length: free_space,
                        free_space: 0, // TODO
                    };
                    defragged_blocks.push(fragged_end_block);

                    let new_end_block = FileBlock {
                        id: end_file_block.id,
                        file_length: end_file_block.file_length - free_space,
                        free_space: 0, // TODO
                    };
                    if new_end_block.file_length > 0 {
                        file_blocks.push_back(new_end_block);
                    }

                    break;
                } else {
                    free_space -= end_file_block.file_length;
                    defragged_blocks.push(FileBlock {
                        id: end_file_block.id,
                        file_length: end_file_block.file_length,
                        free_space: 0,
                    });
                }
            } else {
                break; // end of file blocks
            }
        }
    }

    let mut checksum = 0_i64;
    let mut block_index = 0;
    for block in defragged_blocks {
        for block_i in block_index..(block_index + block.file_length) {
            checksum += (block_i * block.id) as i64;
        }
        block_index += block.file_length;
    }

    Ok(checksum)
}

fn calc_whole_file_checksum(filename: &str) -> io::Result<i64> {
    let file_blocks = fetch_file_blocks(filename)?;
    let mut file_blocks = VecDeque::from(file_blocks);
    let capacity = file_blocks.len();

    let mut finalised_blocks = vec![];
    let mut unmoveable_blocks = vec![];

    while let Some(file_block) = file_blocks.pop_back() {
        let mut new_file_blocks = VecDeque::with_capacity(capacity);

        let mut found_space = false;

        // find space
        while let Some(possible_block) = file_blocks.pop_front() {
            if possible_block.free_space >= file_block.file_length {
                found_space = true;
                new_file_blocks.push_back(FileBlock {
                    id: possible_block.id,
                    file_length: possible_block.file_length,
                    free_space: 0,
                });

                new_file_blocks.push_back(FileBlock {
                    id: file_block.id,
                    file_length: file_block.file_length,
                    free_space: possible_block.free_space - file_block.file_length,
                });

                // pad out previous block to preserve index
                if let Some(padding_block) = file_blocks.pop_back() {
                    file_blocks.push_back(FileBlock {
                        id: padding_block.id,
                        file_length: padding_block.file_length,
                        free_space: padding_block.free_space
                            + file_block.file_length
                            + file_block.free_space,
                    });
                }

                while let Some(b) = file_blocks.pop_front() {
                    new_file_blocks.push_back(b);
                }
            } else {
                new_file_blocks.push_back(possible_block);
            }
        }
        if found_space {
            // blocks with no free space are no longer interesting. this gives a minor performance improvement
            while let Some(block) = new_file_blocks.pop_front() {
                if block.free_space == 0 {
                    finalised_blocks.push(block);
                } else {
                    new_file_blocks.push_front(block);
                    break;
                }
            }
        } else {
            unmoveable_blocks.push(file_block);
        }
        file_blocks = new_file_blocks;
    }

    let mut checksum = 0_i64;
    let mut block_index = 0;
    unmoveable_blocks.reverse();
    for block in finalised_blocks.into_iter().chain(unmoveable_blocks) {
        for block_i in block_index..(block_index + block.file_length) {
            checksum += (block_i * block.id) as i64;
        }
        block_index += block.file_length + block.free_space;
    }

    Ok(checksum)
}

struct FileBlock {
    id: i32,
    file_length: i32,
    free_space: i32,
}

fn fetch_file_blocks(filename: &str) -> io::Result<Vec<FileBlock>> {
    let lines = read_lines(filename)?;
    let mut id = 0;
    let mut file_blocks = vec![];
    for line in lines {
        if line.is_empty() {
            continue;
        }

        let cs = line.trim().chars().collect_vec();

        for (file_length, free_space) in cs
            .chunks(2)
            .map(|x| {
                (
                    x[0].to_string().parse::<i32>().unwrap(),
                    x.get(1)
                        .map_or(0, |v| v.to_string().parse::<i32>().unwrap()),
                )
            })
            .collect_vec()
        {
            file_blocks.push(FileBlock {
                id,
                file_length,
                free_space,
            });
            id += 1;
        }
    }
    Ok(file_blocks)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            calc_checksum("./inputs/day-9-input-test.txt").unwrap(),
            1928
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            calc_checksum("./inputs/day-9-input.txt").unwrap(),
            6432869891895
        );
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            calc_whole_file_checksum("./inputs/day-9-input-test.txt").unwrap(),
            2858
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            calc_whole_file_checksum("./inputs/day-9-input.txt").unwrap(),
            6467290479134
        );
    }
}
