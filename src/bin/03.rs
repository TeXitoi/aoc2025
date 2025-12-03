fn max_joltage(l: &[u8], num: usize) -> anyhow::Result<u64> {
    anyhow::ensure!(l.len() >= num && l.iter().all(|&j| j < 10));
    let mut sum = 0;
    let mut fst = 0;
    for i in (0..num).rev() {
        let max = *l[fst..l.len() - i].iter().max().unwrap();
        fst = l[fst..l.len() - i].iter().position(|i| *i == max).unwrap() + fst + 1;
        sum = sum * 10 + u64::from(max);
    }
    Ok(sum)
}

fn main() -> anyhow::Result<()> {
    let mut sum2 = 0;
    let mut sum12 = 0;
    for l in std::fs::read_to_string("data/example03.txt")?.lines() {
        let l: Vec<u8> = l.as_bytes().iter().map(|i| i - b'0').collect();
        sum2 += max_joltage(&l, 2)?;
        sum12 += max_joltage(&l, 12)?;
    }
    println!("Part1: {sum2}");
    println!("Part2: {sum12}");
    Ok(())
}
