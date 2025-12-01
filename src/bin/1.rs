use aoc2025::*;

fn main() -> anyhow::Result<()> {
    let mut cur = 50;
    let mut nb_zero = 0;
    let mut nb_pass_zero = 0;
    for l in lines("data/example1.txt")? {
        let l = l?;
        let incr = match l.chars().next() {
            Some('R') => 1,
            Some('L') => -1,
            _ => anyhow::bail!("bad line {l}"),
        };
        for _ in 0..l[1..].parse()? {
            cur = (cur + 100 + incr) % 100;
            nb_pass_zero += i32::from(cur == 0);
        }
        nb_zero += i32::from(cur == 0);
    }
    println!("Part1: {}", nb_zero);
    println!("Part2: {}", nb_pass_zero);
    Ok(())
}
