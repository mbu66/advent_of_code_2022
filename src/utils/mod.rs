use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

#[allow(dead_code)]
pub fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}