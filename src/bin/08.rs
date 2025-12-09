// see https://en.wikipedia.org/wiki/Disjoint-set_data_structure
struct DisjointSet {
    parent: Vec<usize>,
}
impl DisjointSet {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }
    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            self.parent[i] = self.find(self.parent[i]);
            self.parent[i]
        }
    }
    fn union(&mut self, i: usize, j: usize) -> bool {
        let i = self.find(i);
        let j = self.find(j);
        self.parent[j] = i;
        i != j
    }
}

fn main() -> anyhow::Result<()> {
    let boxes = std::fs::read_to_string("data/example08.txt")?
        .lines()
        .map(|l| {
            let &[x, y, z] = l
                .split(',')
                .map(|c| c.parse())
                .collect::<Result<Vec<i64>, _>>()?
                .as_slice()
            else {
                anyhow::bail!("bad line {l:?}")
            };
            Ok((x, y, z))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    let mut edges: Vec<_> = (0..boxes.len() - 1)
        .flat_map(|i| (i + 1..boxes.len()).map(move |j| (i, j)))
        .map(|(i, j)| {
            let bi = boxes[i];
            let bj = boxes[j];
            let sq_distance = (bi.0 - bj.0).pow(2) + (bi.1 - bj.1).pow(2) + (bi.2 - bj.2).pow(2);
            (sq_distance, i, j)
        })
        .collect();

    // basically https://en.wikipedia.org/wiki/Kruskal%27s_algorithm
    edges.sort_unstable();
    let mut disjoint_set = DisjointSet::new(boxes.len());
    let mut num_union = 0;
    for (idx_edge, &(_, i, j)) in edges.iter().enumerate() {
        num_union += usize::from(disjoint_set.union(i, j));
        if num_union == boxes.len() - 1 {
            println!("Part2: {}", boxes[i].0 * boxes[j].0);
            break;
        }
        if [10, 1000].contains(&(idx_edge + 1)) {
            let mut cardinals = vec![0; boxes.len()];
            for i in 0..boxes.len() {
                cardinals[disjoint_set.find(i)] += 1;
            }
            cardinals.sort_unstable();
            cardinals.reverse();
            println!("Part1: {}", cardinals[..3].iter().product::<u64>());
        }
    }
    Ok(())
}
