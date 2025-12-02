fn main() -> anyhow::Result<()> {
    let mut cur: i32 = 50;
    let mut num_zero = 0;
    let mut num_pass_zero = 0;
    for l in std::fs::read_to_string("data/example01.txt")?.lines() {
        let incr = match l.chars().next() {
            Some('R') => 1,
            Some('L') => -1,
            _ => anyhow::bail!("bad line {l:?}"),
        };
        for _ in 0..l[1..].parse()? {
            cur = (cur + incr).rem_euclid(100);
            num_pass_zero += u32::from(cur == 0);
        }
        num_zero += u32::from(cur == 0);
    }
    println!("Part1: {}", num_zero);
    println!("Part2: {}", num_pass_zero);
    Ok(())
}
