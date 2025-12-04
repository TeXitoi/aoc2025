fn around(i: usize, j: usize, grid: &[Vec<u8>]) -> impl Iterator<Item = (usize, usize)> + use<> {
    let up = i.checked_sub(1);
    let left = j.checked_sub(1);
    let down = (i < grid.len() - 1).then_some(i + 1);
    let right = (j < grid[i].len() - 1).then_some(j + 1);
    [
        (up, left),
        (up, Some(j)),
        (up, right),
        (Some(i), left),
        (Some(i), right),
        (down, left),
        (down, Some(j)),
        (down, right),
    ]
    .into_iter()
    .filter_map(|(i, j)| Some((i?, j?)))
}
fn coords(grid: &[Vec<u8>]) -> impl Iterator<Item = (usize, usize)> + use<> {
    let j_len = grid[0].len();
    (0..grid.len()).flat_map(move |i| (0..j_len).map(move |j| (i, j)))
}
fn is_removable((i, j): (usize, usize), g: &[Vec<u8>]) -> bool {
    g[i][j] == b'@' && around(i, j, g).filter(|&(i, j)| g[i][j] == b'@').count() < 4
}
fn main() -> anyhow::Result<()> {
    let mut grid: Vec<_> = std::fs::read_to_string("data/example04.txt")?
        .lines()
        .map(|l| l.trim().as_bytes().to_vec())
        .collect();

    let part1 = coords(&grid).filter(|&c| is_removable(c, &grid)).count();
    println!("Part1: {part1}");

    let mut num = 0;
    let mut found = true;
    while found {
        found = false;
        for (i, j) in coords(&grid) {
            if is_removable((i, j), &grid) {
                num += 1;
                grid[i][j] = b'.';
                found = true;
            }
        }
    }
    println!("Part2: {num}");

    Ok(())
}
