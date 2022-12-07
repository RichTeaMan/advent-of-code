use std::{
    cell::{Ref, RefCell},
    io::{self},
    rc::Rc,
};

use itertools::Itertools;

use crate::file_utils::read_lines;

struct ElfFile {
    pub name: String,
    pub size: usize,
    pub parent: Option<Rc<RefCell<ElfFile>>>,
    pub sub_files: Vec<Rc<RefCell<ElfFile>>>,
}

impl ElfFile {
    pub fn root() -> ElfFile {
        ElfFile {
            name: "/".to_string(),
            parent: None,
            size: 0,
            sub_files: Vec::new(),
        }
    }
    pub fn total_size(&self) -> usize {
        self.size
            + self
                .sub_files
                .iter()
                .map(|f| f.borrow().total_size())
                .sum::<usize>()
    }
    pub fn directory_size(&self) -> usize {
        self.sub_files
            .iter()
            .map(|f| f.borrow().total_size())
            .sum::<usize>()
    }
    pub fn find_file(&self, name: &str) -> Option<Rc<RefCell<ElfFile>>> {
        for sub in &self.sub_files {
            if sub.borrow().name == name {
                return Some(sub.clone());
            }
        }
        None
    }
}

pub fn day_7() -> io::Result<usize> {
    let elf_file = fetch_file_tree("./inputs/day-7-input.txt")?;
    let b = elf_file.borrow();
    Ok(directory_under(100_000, b))
}

pub fn day_7_part_2() -> io::Result<usize> {
    let elf_file = fetch_file_tree("./inputs/day-7-input.txt").unwrap();
    Ok(directory_to_delete(70000000, 30000000, elf_file.borrow()).unwrap())
}

fn directory_to_delete(
    total_disk_space: usize,
    needed_space: usize,
    elf_file: Ref<ElfFile>,
) -> Option<usize> {
    let space_to_free = needed_space - (total_disk_space - elf_file.total_size());
    search_directory_to_delete(space_to_free, elf_file)
}

fn search_directory_to_delete(space_to_free: usize, elf_file: Ref<ElfFile>) -> Option<usize> {
    let mut dir_space_to_delete = None;

    if elf_file.directory_size() > space_to_free {
        dir_space_to_delete = Some(elf_file.directory_size());

        for sub in &elf_file.sub_files {
            if let Some(size) = search_directory_to_delete(space_to_free, sub.borrow()) {
                if size < dir_space_to_delete.unwrap() {
                    dir_space_to_delete = Some(size);
                }
            }
        }
    }
    dir_space_to_delete
}

fn directory_under(max_directory_size: usize, elf_file: Ref<ElfFile>) -> usize {
    let mut size = 0;
    if elf_file.directory_size() < max_directory_size {
        size += elf_file.directory_size();
    }

    for sub in &elf_file.sub_files {
        size += directory_under(max_directory_size, sub.borrow());
    }

    size
}

fn fetch_file_tree(filename: &str) -> io::Result<Rc<RefCell<ElfFile>>> {
    let root = Rc::new(RefCell::new(ElfFile::root()));

    let mut current_dir = root.clone();

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        // cd command
        if line.starts_with("$ cd") {
            let change_dir = line.replace("$ cd ", "");
            if change_dir == ".." {
                let parent_opt = current_dir.borrow().parent.clone();
                if let Some(parent) = parent_opt {
                    current_dir = parent;
                } else {
                    panic!("Trying to get parent of root");
                }
            } else if change_dir == "/" {
                current_dir = root.clone();
            } else {
                // should check for / in cd but I'm bravely choosing not to

                let found_dir_opt = current_dir.borrow().find_file(&change_dir);
                if let Some(found_dir) = found_dir_opt {
                    current_dir = found_dir;
                } else {
                    panic!("Trying to cd to '{change_dir}'.");
                }
            }
        } else if line == "$ ls" {
            // do nothing I guess
        } else if line.starts_with('$') {
            panic!("unrecognised command");
        } else if line.starts_with("dir ") {
            let name = line.replace("dir ", "");

            current_dir
                .as_ref()
                .borrow_mut()
                .sub_files
                .push(Rc::new(RefCell::new(ElfFile {
                    name,
                    size: 0,
                    parent: Some(current_dir.clone()),
                    sub_files: Vec::new(),
                })));
        }
        // parse some actual files
        else if let Some((size_s, name)) = line.split(' ').collect_tuple() {
            let size: usize = size_s.parse().unwrap();

            current_dir
                .as_ref()
                .borrow_mut()
                .sub_files
                .push(Rc::new(RefCell::new(ElfFile {
                    name: name.to_string(),
                    size,
                    parent: Some(current_dir.clone()),
                    sub_files: Vec::new(),
                })));
        } else {
            panic!("Unknown file input.");
        }
    }
    Ok(root)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn total_size_small_test() {
        assert_eq!(
            fetch_file_tree("./inputs/day-7-input-test.txt")
                .unwrap()
                .borrow()
                .total_size(),
            48381165
        );
    }

    #[test]
    fn small_test() {
        let test_elf_file = fetch_file_tree("./inputs/day-7-input-test.txt").unwrap();
        assert_eq!(directory_under(100_000, test_elf_file.borrow()), 95437);
    }

    #[test]
    fn test() {
        assert_eq!(day_7().unwrap(), 1989474,)
    }

    #[test]
    fn part_2_test() {
        let test_elf_file = fetch_file_tree("./inputs/day-7-input.txt").unwrap();
        assert_eq!(
            directory_to_delete(70000000, 30000000, test_elf_file.borrow()).unwrap(),
            1111607
        );
    }
}
