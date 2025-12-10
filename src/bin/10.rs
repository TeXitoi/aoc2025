#[derive(Debug)]
struct Machine {
    lights: u32,
    buttons: Vec<u32>,
    joltages: Vec<i64>,
}
impl Machine {
    fn new(s: &str) -> anyhow::Result<Self> {
        let Some((l, q)) = s.split_once(' ') else {
            anyhow::bail!("bad line {s:?}")
        };
        let lights = l.chars().rev().fold(0, |accu, c| match c {
            '.' => accu << 1,
            '#' => (accu << 1) + 1,
            _ => accu,
        });
        let Some((bs, js)) = q.rsplit_once(' ') else {
            anyhow::bail!("bad line {s:?}")
        };
        let buttons = bs
            .split(' ')
            .map(|b| {
                b[1..b.len() - 1]
                    .split(',')
                    .map(|l| l.parse().unwrap())
                    .fold(0, |accu, b: u32| accu | (1 << b))
            })
            .collect();
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
        //println!("press  {:10b}", press);
        let mut lights = 0;
        for i in 0.. {
            if press == 0 {
                break;
            }
            if press & 1 == 1 {
                lights ^= self.buttons[i];
            }
            press >>= 1;
        }
        //println!("lights {:10b}", lights);
        lights == self.lights
    }
    fn num_buttons(&self) -> Option<u32> {
        //println!("lights {:10b}", self.lights);
        let max = (1 << self.buttons.len()) - 1;
        //println!("max    {:10b}", max);
        (0u32..max)
            .filter(|&bs| self.check_lights(bs))
            .map(|bs| bs.count_ones())
            .min()
    }
}
fn main() -> anyhow::Result<()> {
    let machines = std::fs::read_to_string("data/example10.txt")?
        .lines()
        .map(|l| Machine::new(l))
        .collect::<anyhow::Result<Vec<_>>>()?;
    let num_buttons = machines.iter().filter_map(|m| m.num_buttons()).sum::<u32>();
    println!("Part1: {num_buttons}");
    Ok(())
}
