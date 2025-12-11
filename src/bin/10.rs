use std::collections::BTreeSet;

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<i32>,
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
            .collect::<Result<Vec<Vec<_>>, _>>()?;
        let joltages = js[1..js.len() - 1]
            .split(',')
            .map(|l| l.parse())
            .collect::<Result<Vec<_>, _>>()?;
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
    fn num_buttons(&self) -> Option<u32> {
        (0..1 << self.buttons.len())
            .filter(|&bs| self.check_lights(bs))
            .map(u32::count_ones)
            .min()
    }
    fn min_rem(&self, joltages: &[i32]) -> i32 {
        let mut slots = vec![true; self.joltages.len()];
        let mut min_rem = 0;
        while slots.iter().any(|&s| s) {
            let (max_diff, i) = slots
                .iter()
                .enumerate()
                .filter(|&(_, &s)| s)
                .map(|(i, _)| (self.joltages[i] - joltages[i], i))
                .max()
                .unwrap();
            min_rem += max_diff;
            for b in &self.buttons {
                if b.contains(&i) {
                    for &j in b {
                        slots[j] = false;
                    }
                }
            }
        }
        min_rem
    }
    fn search(&self) -> Option<i32> {
        let init_joltages = vec![0; self.lights.len()];
        let mut nodes = BTreeSet::from([(-dbg!(self.min_rem(&init_joltages)), 0, init_joltages)]);
        let mut seen = BTreeSet::default();
        while let Some((_, num, joltages)) = nodes.pop_last() {
            for button in &self.buttons {
                let mut joltages = joltages.to_vec();
                for &b in button {
                    joltages[b] += 1;
                }
                match self.check_joltages(&joltages) {
                    Some(true) => return Some(num + 1),
                    Some(false) => {}
                    None => {
                        if seen.insert(joltages.clone()) {
                            nodes.insert((-self.min_rem(&joltages) - num - 1, num + 1, joltages));
                        }
                    }
                }
            }
        }
        None
    }
    fn check_joltages(&self, joltages: &[i32]) -> Option<bool> {
        if joltages == self.joltages {
            Some(true)
        } else if self.joltages.iter().zip(joltages).all(|(r, cur)| cur <= r) {
            None
        } else {
            Some(false)
        }
    }
    fn solve(&self) -> i32 {
        use good_lp::{Expression, ProblemVariables, Solution, SolverModel, variable};
        let mut problem = ProblemVariables::new();
        let variables: Vec<_> = self
            .buttons
            .iter()
            .map(|_| problem.add(variable().integer().min(0)))
            .collect();
        let objective = variables.iter().sum::<Expression>();
        let mut problem = problem.minimise(&objective).using(good_lp::highs);
        for (i, &target) in self.joltages.iter().enumerate() {
            problem.add_constraint(
                self.buttons
                    .iter()
                    .enumerate()
                    .filter(|(_, b)| b.contains(&i))
                    .map(|(i, _)| variables[i])
                    .sum::<Expression>()
                    .eq(target),
            );
        }
        let solution = problem.solve().unwrap();
        solution.eval(objective).round() as i32
    }
}
fn main() -> anyhow::Result<()> {
    let mut machines = std::fs::read_to_string("data/input10.txt")?
        .lines()
        .map(Machine::new)
        .collect::<anyhow::Result<Vec<_>>>()?;
    machines.sort_unstable_by_key(|m| m.buttons.len());

    let num_buttons = machines.iter().filter_map(|m| m.num_buttons()).sum::<u32>();
    println!("Part1: {num_buttons}");

    //let num: i32 = machines.iter().map(|m| dbg!(m.solve())).sum();
    //println!("Part2: {num}");
    let num: i32 = machines
        .iter()
        .map(|m| {
            let num = m.search().unwrap();
            assert_eq!(num, m.solve());
            dbg!(num)
        })
        .sum();
    println!("Part2: {num}");
    Ok(())
}
