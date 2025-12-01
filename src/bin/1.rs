use std::io::{self, BufRead};

fn main() -> anyhow::Result<()> {
    let mut cur = 50;
    let mut nb_zero = 0;
    let mut nb_pass_zero = 0;
    for l in io::BufReader::new(std::fs::File::open("data/example1.txt")?).lines() {
        let l = l?;
        let n = l[1..].parse::<i32>()?;
        match l.chars().next() {
            Some('R') => {
                for _ in 0..n {
                    cur = (cur + 1) % 100;
                    nb_pass_zero += i32::from(cur == 0);
                }
            }
            Some('L') => {
                for _ in 0..n {
                    cur = (cur + 100 - 1) % 100;
                    nb_pass_zero += i32::from(cur == 0);
                }
            }
            _ => anyhow::bail!("bad line {l}"),
        }
        nb_zero += i32::from(cur == 0);
    }
    println!("Part1: {}", nb_zero);
    println!("Part2: {}", nb_pass_zero);
    Ok(())
}
