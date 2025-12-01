use std::fs::File;
use std::io::{self, BufRead};

pub fn lines(file: &str) -> io::Result<Vec<String>> {
    io::BufReader::new(File::open(file)?).lines().collect()
}
