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
    fn min_rem(&self, joltages: &[i32]) -> i32 {
        self.joltages
            .iter()
            .zip(joltages)
            .map(|(j1, j2)| j1 - j2)
            .max()
            .unwrap_or(0)
    }
    fn search(&self) -> Option<i32> {
        let init_joltages = vec![0; self.lights.len()];
        let mut nodes = BTreeSet::from([(-self.min_rem(&init_joltages), 0, init_joltages)]);
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
        let pushes: Vec<_> = variables
            .iter()
            .map(|&v| solution.eval(v).round() as u32)
            .collect();
        //println!("pushes: {pushes:?}");
        solution.eval(objective).round() as i32
    }
}
fn main() -> anyhow::Result<()> {
    let mut machines = std::fs::read_to_string("data/input10.txt")?
        .lines()
        .map(|l| Machine::new(l))
        .collect::<anyhow::Result<Vec<_>>>()?;
    machines.sort_unstable_by_key(|m| m.buttons.len());
    let num: i32 = machines
        .iter_mut()
        .map(|m| {
            let num = m.search().unwrap();
            assert_eq!(num, m.solve());
            dbg!(num)
        })
        .sum();
    println!("Part2: {num}");
    Ok(())
}
