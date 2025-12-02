pub use std::fs::File;
pub use std::io::{self, BufRead, Read};

pub fn lines(file: &str) -> io::Result<Vec<String>> {
    io::BufReader::new(File::open(file)?).lines().collect()
}
