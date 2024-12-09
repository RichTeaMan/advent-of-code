use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

/**
 * Returns file lines in an iterator.
 */
pub fn read_lines<P>(filename: P) -> io::Result<impl Iterator<Item = String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().map_while(Result::ok))
}
