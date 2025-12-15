struct Problem {
    height: usize,
    width: usize,
    num_by_present: Vec<usize>,
}
impl std::str::FromStr for Problem {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        let Some((box_size, num_by_presents)) = s.split_once(' ') else {
            anyhow::bail!("bad problem {s:?}");
        };
        let Some((h, w)) = box_size[..box_size.len() - 1].split_once('x') else {
            anyhow::bail!("bad problem {s:?}");
        };
        Ok(Self {
            height: h.parse()?,
            width: w.parse()?,
            num_by_present: num_by_presents
                .split(' ')
                .map(|num| num.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}
impl Problem {
    fn has_trivial_solution(&self) -> bool {
        self.num_by_present.iter().sum::<usize>() <= (self.height / 3) * (self.width / 3)
    }
    fn is_unfeasible(&self, presents: &[Present]) -> bool {
        let present_area = presents
            .iter()
            .zip(&self.num_by_present)
            .map(|(p, &n)| p.area() * n)
            .sum();
        self.height * self.width < present_area
    }
}

struct Present(Vec<Vec<bool>>);
impl Present {
    fn area(&self) -> usize {
        self.0.iter().flatten().filter(|&&b| b).count()
    }
}
impl std::str::FromStr for Present {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        let p: Vec<Vec<bool>> = s
            .split('\n')
            .skip(1)
            .map(|l| l.chars().map(|c| c == '#').collect())
            .collect();
        anyhow::ensure!(p.len() == 3 && p.iter().all(|p| p.len() == 3));
        Ok(Self(p))
    }
}

fn main() -> anyhow::Result<()> {
    let data = std::fs::read_to_string("data/input12.txt")?;
    let blocks: Vec<_> = data.split("\n\n").collect();
    let problems = blocks
        .last()
        .ok_or(anyhow::anyhow!("bad file"))?
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Problem>, _>>()?;
    let presents = blocks[..blocks.len() - 1]
        .iter()
        .map(|b| b.parse())
        .collect::<Result<Vec<Present>, _>>()?;
    let lower = problems.iter().filter(|p| p.has_trivial_solution()).count();
    let upper = problems
        .iter()
        .filter(|p| !p.is_unfeasible(&presents))
        .count();
    print!("Part1 in [{lower}, {upper}]");
    Ok(())
}
