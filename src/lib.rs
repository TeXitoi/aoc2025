use std::io::{self, BufRead};

pub fn lines(file: &str) -> anyhow::Result<impl Iterator<Item = io::Result<String>>> {
    Ok(std::io::BufReader::new(std::fs::File::open(file)?).lines())
}
