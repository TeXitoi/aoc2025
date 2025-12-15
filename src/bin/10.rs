use std::collections::BTreeMap as Map;
use std::io::Write;

type Joltage = u8;
type Joltages = MuVec<Joltage>;

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct MuVec<T: Default> {
    data: [T; 10],
    len: u8,
}
impl<T: Default> MuVec<T> {
    pub fn push(&mut self, e: T) {
        assert!(self.len < 10);
        self.data[usize::from(self.len)] = e;
        self.len += 1;
    }
    pub fn as_slice(&self) -> &[T] {
        &self.data[..self.len as usize]
    }
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data[..self.len as usize]
    }
}
impl<T: Default> std::ops::Deref for MuVec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}
impl<T: Default> std::ops::DerefMut for MuVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}
impl<T: std::fmt::Debug + Default> std::fmt::Debug for MuVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_slice().fmt(f)
    }
}
impl<T: Default> std::iter::FromIterator<T> for MuVec<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut res = Self::default();
        for j in iter {
            res.push(j);
        }
        res
    }
}
impl<T: Default, const N: usize> From<[T; N]> for MuVec<T> {
    fn from(v: [T; N]) -> Self {
        v.into_iter().collect()
    }
}

#[derive(Debug)]
struct Machine {
    line: String,
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Joltages,
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
            .collect::<Result<Joltages, _>>()?;
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
    fn min_rem(&self, joltages: &[Joltage]) -> u16 {
        let mut slots: MuVec<_> = self.joltages.iter().map(|_| true).collect();
        let mut min_rem = 0;
        while slots.iter().any(|&s| s) {
            let (max_diff, i) = slots
                .iter()
                .enumerate()
                .filter(|&(_, &s)| s)
                .map(|(i, _)| (u16::from(self.joltages[i]) - u16::from(joltages[i]), i))
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
    fn make_permu(&self) -> Vec<usize> {
        let mut buttons = self.buttons.clone();
        let mut permu = vec![];
        while !buttons.is_empty() {
            let (_, _, smallest_j) = (0..self.joltages.len())
                .map(|i| {
                    (
                        buttons.iter().filter(|b| b.contains(&i)).count(),
                        -(buttons
                            .iter()
                            .filter(|b| b.contains(&i))
                            .map(|b| b.len() as isize)
                            .sum::<isize>()),
                        i,
                    )
                })
                .filter(|&(n, _, _)| n != 0)
                .min()
                .unwrap();
            permu.push(smallest_j);
            buttons.retain(|b| !b.contains(&smallest_j));
        }
        for i in 0..self.joltages.len() {
            if !permu.contains(&i) {
                permu.push(i);
            }
        }
        permu
    }
    fn rewrite(&self) -> Self {
        let permu = self.make_permu();
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
    fn search(&self) -> u16 {
        let mach = self.rewrite();
        dbg!(&mach.line);
        let mut best = u16::MAX;
        let mut cur = Map::default();
        let mut next = Map::from([(vec![0; mach.joltages.len()], 0)]);
        for (button_idx, button) in mach.buttons.iter().enumerate() {
            print!("{}.", next.len());
            std::io::stdout().flush().unwrap();
            (cur, next) = (next, cur);
            for (joltages, num) in std::mem::replace(&mut cur, Default::default()) {
                for i in 0.. {
                    let num = num + u16::from(i);
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
    fn check_until_button(&self, button_idx: usize, joltages: &[Joltage]) -> bool {
        if button_idx >= self.buttons.len() {
            return false;
        }
        joltages[..self.buttons[button_idx][0]]
            .iter()
            .enumerate()
            .all(|(i, &j)| self.joltages[i] == j)
    }
    fn check_joltages(&self, joltages: &[Joltage]) -> Option<bool> {
        if *joltages == *self.joltages {
            Some(true)
        } else if self.joltages.iter().zip(joltages).all(|(r, cur)| cur <= r) {
            None
        } else {
            Some(false)
        }
    }
    fn solve(&self) -> u16 {
        use microlp::{ComparisonOp::Eq, OptimizationDirection, Problem};
        let mut problem = Problem::new(OptimizationDirection::Minimize);
        let variables: Vec<_> = self
            .buttons
            .iter()
            .map(|_| problem.add_integer_var(1., (0, i32::MAX)))
            .collect();
        for (i, &target) in self.joltages.iter().enumerate() {
            problem.add_constraint(
                self.buttons
                    .iter()
                    .zip(&variables)
                    .filter(|(b, _)| b.contains(&i))
                    .map(|(_, &v)| (v, 1.)),
                Eq,
                f64::from(target),
            );
        }
        let solution = problem.solve().unwrap();
        solution.objective().round() as u16
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

    let num: u16 = machines.iter().map(|m| m.solve()).sum();
    println!("Part2: {num}");
    dbg!(machines.len());
    let num: u16 = machines
        .iter()
        .rev()
        .enumerate()
        .map(|(i, m)| {
            dbg!(i);
            let num1 = dbg!(m.solve());
            let num2 = dbg!(m.search());
            assert_eq!(num1, num2);
            num2
        })
        .sum();
    println!("Part2: {num}");
    Ok(())
}
