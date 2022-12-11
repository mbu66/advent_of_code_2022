use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

#[allow(dead_code)]
pub fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

#[allow(dead_code)]
pub fn chrono<F: Fn() -> T, T>(f: F) -> T {
    let now = std::time::Instant::now();
    let result = f();
    print!("Time Taken = {}ms\n", now.elapsed().as_millis());
    result
  }