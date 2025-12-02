use aoc2025::*;

fn check(s: &[u8], len: usize) -> bool {
    s.len().is_multiple_of(len) && s[len..].chunks(len).all(|n| n == &s[..len])
}

fn main() -> anyhow::Result<()> {
    let mut data = String::new();
    File::open("data/example02.txt")?.read_to_string(&mut data)?;
    let mut sum1 = 0;
    let mut sum2 = 0;
    for interval in data.split(',') {
        let v = interval
            .split('-')
            .map(|s| s.trim().parse())
            .collect::<Result<Vec<u64>, _>>()?;
        let &[start, end] = v.as_slice() else {
            anyhow::bail!("bad interval {interval:?}")
        };
        for i in start..=end {
            let s = i.to_string().into_bytes();
            if check(&s, s.len() / 2) {
                sum1 += i;
            }
            if (1..=s.len() / 2).any(|len| check(&s, len)) {
                sum2 += i;
            }
        }
    }
    println!("Part1: {}", sum1);
    println!("Part2: {}", sum2);
    Ok(())
}
