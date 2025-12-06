fn main() -> anyhow::Result<()> {
    let raw_data = std::fs::read_to_string("data/input06.txt")?;
    let data: Vec<Vec<_>> = raw_data
        .lines()
        .map(|l| l.split(' ').filter(|s| !s.is_empty()).collect())
        .collect();
    let numbers = data[..data.len() - 1]
        .iter()
        .map(|l| l.iter().map(|n| n.parse()).collect())
        .collect::<Result<Vec<Vec<u64>>, _>>()?;
    let ops = &data[data.len() - 1];
    let sum: u64 = ops
        .iter()
        .enumerate()
        .map(|(i, &op)| -> u64 {
            let numbers = numbers.iter().map(|ns| ns[i]);
            match op {
                "+" => numbers.sum(),
                "*" => numbers.product(),
                _ => unreachable!(),
            }
        })
        .sum();
    println!("Part1: {sum}");

    let data: Vec<&[u8]> = raw_data.lines().map(|l| l.as_bytes()).collect();
    let data = &data[..];
    let stream = (0..data[0].len())
        .rev()
        .flat_map(move |j| (0..data.len()).map(move |i| char::from(data[i][j])));
    let mut sum = 0;
    let mut s = String::new();
    for c in stream {
        match c {
            '*' => {
                sum += s
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|n| n.parse::<u64>().unwrap())
                    .product::<u64>();
                s.clear();
            }
            '+' => {
                sum += s
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|n| n.parse::<u64>().unwrap())
                    .sum::<u64>();
                s.clear();
            }
            c => s.push(c),
        }
    }
    println!("Part2: {sum}");

    Ok(())
}
