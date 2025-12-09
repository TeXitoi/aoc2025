type Coord = (i64, i64);
#[derive(Copy, Clone)]
struct Rect {
    min_x: i64,
    min_y: i64,
    max_x: i64,
    max_y: i64,
}
impl Rect {
    fn new(coords: &[Coord]) -> Self {
        Self {
            min_x: coords[0].0.min(coords[1].0),
            min_y: coords[0].1.min(coords[1].1),
            max_x: coords[0].0.max(coords[1].0),
            max_y: coords[0].1.max(coords[1].1),
        }
    }
    fn intersect(self, other: Self) -> bool {
        !(self.max_x <= other.min_x
            || other.max_x <= self.min_x
            || self.max_y <= other.min_y
            || other.max_y <= self.min_y)
    }
    fn area(self) -> i64 {
        (self.max_x - self.min_x + 1) * (self.max_y - self.min_y + 1)
    }
}
fn main() -> anyhow::Result<()> {
    let coords = std::fs::read_to_string("data/example09.txt")?
        .lines()
        .map(|l| {
            let Some((x, y)) = l.split_once(',') else {
                anyhow::bail!("bad line {l:?}")
            };
            Ok((x.parse()?, y.parse()?))
        })
        .collect::<anyhow::Result<Vec<Coord>>>()?;
    let segments: Vec<_> = coords
        .windows(2)
        .chain([[coords[0], coords[coords.len() - 1]].as_slice()])
        .map(Rect::new)
        .collect();
    for i in [1, 2] {
        let max = coords
            .iter()
            .enumerate()
            .flat_map(|(i, &c1)| coords[i + 1..].iter().map(move |&c2| Rect::new(&[c1, c2])))
            .filter(|&r| i == 1 || segments.iter().all(|&segment| !r.intersect(segment)))
            .map(|r| r.area())
            .max()
            .unwrap_or(0);
        println!("Part{i}: {max}");
    }
    Ok(())
}
