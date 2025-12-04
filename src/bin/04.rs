type Coord = (usize, usize);

struct Grid(Vec<Vec<u8>>);
impl Grid {
    fn new(f: &str) -> anyhow::Result<Self> {
        Ok(Self(
            std::fs::read_to_string(f)?
                .lines()
                .map(|l| l.trim().as_bytes().to_vec())
                .collect(),
        ))
    }
    fn around(&self, (i, j): Coord) -> impl Iterator<Item = Coord> + use<> {
        let up = i.checked_sub(1);
        let left = j.checked_sub(1);
        let down = (i < self.0.len() - 1).then_some(i + 1);
        let right = (j < self.0[i].len() - 1).then_some(j + 1);
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
    fn coords(&self) -> impl Iterator<Item = Coord> + use<> {
        let j_len = self.0[0].len();
        (0..self.0.len()).flat_map(move |i| (0..j_len).map(move |j| (i, j)))
    }
    fn is_roll(&self, (i, j): Coord) -> bool {
        self.0[i][j] == b'@'
    }
    fn is_removable(&self, coord: Coord) -> bool {
        self.is_roll(coord)
            && self
                .around(coord)
                .filter(|&coord| self.is_roll(coord))
                .count()
                < 4
    }
    fn remove(&mut self, (i, j): Coord) {
        self.0[i][j] = b'.';
    }
}

fn main() -> anyhow::Result<()> {
    let mut grid = Grid::new("data/example04.txt")?;

    let part1 = grid.coords().filter(|&c| grid.is_removable(c)).count();
    println!("Part1: {part1}");

    let mut num = 0;
    let mut removed = true;
    while removed {
        removed = false;
        for coord in grid.coords() {
            if grid.is_removable(coord) {
                num += 1;
                grid.remove(coord);
                removed = true;
            }
        }
    }
    println!("Part2: {num}");

    Ok(())
}
