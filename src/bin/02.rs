fn check(s: &[u8], len: usize) -> bool {
    s.len().is_multiple_of(len) && s[len..].chunks(len).all(|n| n == &s[..len])
}

fn main() -> anyhow::Result<()> {
    let mut sum1 = 0;
    let mut sum2 = 0;
    for interval in std::fs::read_to_string("data/input02.txt")?.split(',') {
        let Some((start, end)) = interval.split_once('-') else {
            anyhow::bail!("bad line {interval:?}")
        };
        for i in start.parse::<u64>()?..=end.parse()? {
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
