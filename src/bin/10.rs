use std::collections::BTreeMap;
use std::io::Write;

#[derive(Debug)]
struct Machine {
    line: String,
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
            line: s.into(),
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
    fn rewrite(&self) -> Self {
        let mut permu: Vec<_> = (0..self.joltages.len()).collect();
        permu.sort_unstable_by_key(|i| self.buttons.iter().filter(|b| b.contains(i)).count());
        //permu.sort_unstable_by_key(|&i| self.joltages[i]);
        let lights = permu.iter().map(|&i| self.lights[i]).collect();
        let joltages = permu.iter().map(|&i| self.joltages[i]).collect();
        let mut buttons: Vec<Vec<_>> = self
            .buttons
            .iter()
            .map(|b| {
                b.iter()
                    .map(|&i| permu.iter().position(|&j| j == i).unwrap())
                    .collect()
            })
            .collect();
        buttons.iter_mut().for_each(|b| b.sort_unstable());
        buttons.sort_unstable();
        let line = format!("{:?} {:?}", buttons, joltages);
        Self {
            line,
            lights,
            buttons,
            joltages,
        }
    }
    fn search(&self) -> i32 {
        let mach = self.rewrite();
        dbg!(&mach.line);
        let mut best = i32::MAX;
        let mut cur = BTreeMap::default();
        let mut next = BTreeMap::from([(vec![0; mach.joltages.len()], 0)]);
        for (button_idx, button) in mach.buttons.iter().enumerate() {
            print!(".");
            std::io::stdout().flush().unwrap();
            (cur, next) = (next, cur);
            for (joltages, num) in std::mem::replace(&mut cur, Default::default()) {
                for i in 0.. {
                    let num = num + i;
                    let mut new_joltages = joltages.clone();
                    for &j in button {
                        new_joltages[j] += i;
                    }
                    match mach.check_joltages(&new_joltages) {
                        Some(true) => {
                            best = best.min(num);
                            //dbg!(best);
                            break;
                        }
                        Some(false) => break,
                        None if num + mach.min_rem(&new_joltages) < best
                            && mach.check_until_button(button_idx + 1, &new_joltages) =>
                        {
                            let n = next.entry(new_joltages).or_insert(num);
                            *n = num.min(*n);
                        }
                        None => {}
                    }
                }
            }
        }
        println!("");
        best
    }
    fn check_until_button(&self, button_idx: usize, joltages: &[i32]) -> bool {
        if button_idx >= self.buttons.len() {
            return false;
        }
        joltages[..self.buttons[button_idx][0]]
            .iter()
            .enumerate()
            .all(|(i, &j)| self.joltages[i] == j)
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
    dbg!(machines.len());
    let num: i32 = machines
        .iter()
        .enumerate()
        .map(|(i, m)| {
            dbg!(i);
            let num = dbg!(m.search());
            assert_eq!(num, m.solve());
            num
        })
        .sum();
    println!("Part2: {num}");
    Ok(())
}
