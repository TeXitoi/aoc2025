fn jolts(mut l: &[u8], num: usize) -> u64 {
    let mut res = 0;
    for i in (0..num).rev() {
        let max = *l[..l.len() - i].iter().max().unwrap();
        l = &l[l.iter().position(|&i| i == max).unwrap() + 1..];
        res = res * 10 + u64::from(max);
    }
    res
}

fn main() -> anyhow::Result<()> {
    let lines: Vec<Vec<_>> = std::fs::read_to_string("data/example03.txt")?
        .lines()
        .map(|l| l.trim().as_bytes().iter().map(|i| i - b'0').collect())
        .collect();
    println!("Part1: {}", lines.iter().map(|l| jolts(l, 2)).sum::<u64>());
    println!("Part2: {}", lines.iter().map(|l| jolts(l, 12)).sum::<u64>());
    Ok(())
}
