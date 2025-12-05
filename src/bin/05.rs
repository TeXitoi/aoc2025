fn main() -> anyhow::Result<()> {
    let data = std::fs::read_to_string("data/input05.txt")?;
    let mut lines = data.lines();
    let mut ranges = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let Some((start, end)) = l.split_once('-') else {
                anyhow::bail!("bad line {l:?}")
            };
            Ok((start.parse()?, end.parse()?))
        })
        .collect::<anyhow::Result<Vec<(u64, u64)>>>()?;
    ranges.sort_unstable();
    ranges.dedup_by(|r2, r1| {
        if r1.1 < r2.0 {
            false
        } else {
            r1.1 = r1.1.max(r2.1);
            true
        }
    });

    let num = lines
        .map(|l| l.parse().unwrap())
        .filter(|&i| ranges.iter().any(|r| r.0 <= i && i <= r.1))
        .count();
    println!("Part1: {num}");

    let len: u64 = ranges.iter().map(|r| r.1 - r.0 + 1).sum();
    println!("Part2: {len}");

    Ok(())
}
