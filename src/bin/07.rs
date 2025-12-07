fn main() -> anyhow::Result<()> {
    let grid: Vec<_> = std::fs::read_to_string("data/example07.txt")?
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect();
    let Some(start) = grid[0].iter().position(|&c| c == b'S') else {
        anyhow::bail!("No start");
    };
    let mut num_split = 0;
    let mut nums = vec![vec![0; grid[0].len()]; grid.len()];
    nums[0][start] = 1;
    for i in 0..grid.len() - 1 {
        for j in 0..grid[0].len() {
            if grid[i][j] == b'^' {
                num_split += u32::from(nums[i][j] != 0);
                nums[i + 1][j - 1] += nums[i][j];
                nums[i + 1][j + 1] += nums[i][j];
            } else {
                nums[i + 1][j] += nums[i][j];
            }
        }
    }
    println!("Part1: {}", num_split);
    println!("Part2: {}", nums[nums.len() - 1].iter().sum::<u64>());
    Ok(())
}
