use std::io::{self, BufRead};

fn main() -> anyhow::Result<()> {
    let mut cur = 50;
    let mut nb_zero = 0;
    for l in io::BufReader::new(std::fs::File::open("data/example1.txt")?).lines() {
        let l = l?;
        let n = l[1..].parse::<i32>()?;
        match l.chars().next() {
            Some('R') => cur = (cur + n) % 100,
            Some('L') => cur = (cur + 100 - n) % 100,
            _ => anyhow::bail!("bad line {l}"),
        }
        if cur == 0 {
            nb_zero += 1;
        }
    }
    println!("Part1: {}", nb_zero);
    Ok(())
}
