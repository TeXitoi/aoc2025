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
    fn search(&self) -> Option<u32> {
        let mut nodes = BTreeSet::from([(0, 0, vec![0; self.lights.len()])]);
        let mut last = 0;
        let mut seen = BTreeSet::default();
        while let Some((num, _, joltages)) = nodes.pop_first() {
            if last != num {
                last = num;
                dbg!(num);
                dbg!(nodes.len());
                dbg!(seen.len());
            }
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
                            nodes.insert((num + 1, -joltages.iter().sum::<i32>(), joltages));
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
    fn solve(&self) -> Option<u32> {
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
        Some(solution.eval(objective).round() as u32)
    }
}
fn main() -> anyhow::Result<()> {
    let mut machines = std::fs::read_to_string("data/input10.txt")?
        .lines()
        .map(|l| Machine::new(l))
        .collect::<anyhow::Result<Vec<_>>>()?;
    let num: u32 = machines.iter_mut().map(|m| dbg!(m.solve().unwrap())).sum();
    println!("Part2: {num}");
    Ok(())
}
