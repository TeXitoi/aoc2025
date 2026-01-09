use std::collections::BTreeMap as Map;

type Edges<'a> = Map<&'a str, Vec<&'a str>>;
type NumPaths<'a> = Map<&'a str, u64>;

fn dfs<'a>(from: &'a str, num_paths: &mut NumPaths<'a>, edges: &Edges<'a>) {
    num_paths.entry(from).or_default();
    for &to in edges.get(from).into_iter().flatten() {
        if !num_paths.contains_key(to) {
            dfs(to, num_paths, edges);
        }
        *num_paths.get_mut(from).unwrap() += num_paths[to];
    }
}
fn num_paths<'a>(from: &'a str, tos: impl Into<NumPaths<'a>>, edges: &Edges<'a>) -> u64 {
    let mut num_paths = tos.into();
    dfs(from, &mut num_paths, edges);
    num_paths[from]
}

fn main() -> anyhow::Result<()> {
    let data = std::fs::read_to_string("data/example11.txt")?;
    let edges = data
        .lines()
        .map(|l| {
            let Some((from, tos)) = l.split_once(' ') else {
                anyhow::bail!("bad line {l:?}")
            };
            let tos = tos.split(' ').collect();
            Ok((&from[..from.len() - 1], tos))
        })
        .collect::<Result<Edges<'_>, _>>()?;

    let you = num_paths("you", [("out", 1)], &edges);
    println!("Part1: {}", you);

    let fft = num_paths("fft", [("out", 1)], &edges);
    let dac_fft = num_paths("dac", [("fft", fft)], &edges);
    let dac = num_paths("dac", [("out", 1)], &edges);
    let fft_dac = num_paths("fft", [("dac", dac)], &edges);
    let svr = num_paths("svr", [("fft", fft_dac), ("dac", dac_fft)], &edges);
    println!("Part2: {}", svr);

    Ok(())
}
