struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u8>,
}
impl Machine {
    fn new(s: &str) -> anyhow::Result<Self> {
        let Some((l, q)) = s.split_once(' ') else {
            anyhow::bail!("bad line {s:?}")
        };
        let lights = l
            .chars()
            .filter_map(|c| match c {
                '.' => Some(false),
                '#' => Some(true),
                _ => None,
            })
            .collect();
        let Some((bs, js)) = q.rsplit_once(' ') else {
            anyhow::bail!("bad line {s:?}")
        };
        let buttons = bs
            .split(' ')
            .map(|b| b[1..b.len() - 1].split(',').map(|l| l.parse()).collect())
            .collect::<Result<_, _>>()?;
        let joltages = js[1..js.len() - 1]
            .split(',')
            .map(|l| l.parse())
            .collect::<Result<_, _>>()?;
        Ok(Self {
            lights,
            buttons,
            joltages,
        })
    }
    fn check_lights(&self, mut press: u32) -> bool {
        let mut lights = vec![false; self.lights.len()];
        for i in 0.. {
            if press == 0 {
                break;
            } else if press & 1 == 1 {
                for &j in &self.buttons[i] {
                    lights[j] = !lights[j];
                }
            }
            press >>= 1;
        }
        lights == self.lights
    }
    fn num1(&self) -> u32 {
        (0..1 << self.buttons.len())
            .filter(|&bs| self.check_lights(bs))
            .map(u32::count_ones)
            .min()
            .unwrap()
    }
    fn num2(&self) -> u16 {
        use microlp::{ComparisonOp, OptimizationDirection, Problem};
        let mut problem = Problem::new(OptimizationDirection::Minimize);
        let variables: Vec<_> = self
            .buttons
            .iter()
            .map(|_| problem.add_integer_var(1., (0, i32::MAX)))
            .collect();
        for (i, &target) in self.joltages.iter().enumerate() {
            problem.add_constraint(
                variables
                    .iter()
                    .copied()
                    .zip(self.buttons.iter().map(|b| b.contains(&i).into())),
                ComparisonOp::Eq,
                f64::from(target),
            );
        }
        problem.solve().unwrap().objective().round() as u16
    }
}
fn main() -> anyhow::Result<()> {
    let machines = std::fs::read_to_string("data/input10.txt")?
        .lines()
        .map(Machine::new)
        .collect::<anyhow::Result<Vec<_>>>()?;
    println!("Part1: {}", machines.iter().map(|m| m.num1()).sum::<u32>());
    println!("Part2: {}", machines.iter().map(|m| m.num2()).sum::<u16>());
    Ok(())
}
