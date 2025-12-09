type Coord = (i64, i64);
fn transpose((x, y): Coord) -> Coord {
    (y, x)
}
fn intersect(mut r: (Coord, Coord), mut s: (Coord, Coord)) -> bool {
    if s.0.1 == s.1.1 {
        r = (transpose(r.0), transpose(r.1));
        s = (transpose(s.0), transpose(s.1));
    }
    let min_r_x = r.0.0.min(r.1.0);
    let max_r_x = r.0.0.max(r.1.0);
    let min_r_y = r.0.1.min(r.1.1);
    let max_r_y = r.0.1.max(r.1.1);
    let s_x = s.0.0;
    let min_s_y = s.0.1.min(s.1.1);
    let max_s_y = s.0.1.max(s.1.1);
    if s_x <= min_r_x || max_r_x <= s_x {
        return false;
    }
    if max_s_y <= min_r_y || max_r_y <= min_s_y {
        return false;
    }
    true
}
fn has_segment_intersecting_rect(r: (Coord, Coord), coords: &[Coord]) -> bool {
    coords.windows(2).any(|s| intersect(r, (s[0], s[1])))
        || intersect(r, (coords[0], coords[coords.len() - 1]))
}
fn main() -> anyhow::Result<()> {
    let coords = std::fs::read_to_string("data/example09.txt")?
        .lines()
        .map(|l| {
            let Some((x, y)) = l.split_once(',') else {
                anyhow::bail!("bad line {l:?}")
            };
            Ok((x.parse()?, y.parse()?))
        })
        .collect::<anyhow::Result<Vec<Coord>>>()?;
    for i in [1, 2] {
        let max = coords
            .iter()
            .enumerate()
            .flat_map(|(i, &c1)| coords[i + 1..].iter().map(move |&c2| (c1, c2)))
            .filter(|&r| i == 1 || !has_segment_intersecting_rect(r, &coords))
            .map(|(c1, c2)| ((c1.0 - c2.0).abs() + 1) * ((c1.1 - c2.1).abs() + 1))
            .max()
            .unwrap_or(0);
        println!("Part{i}: {max}");
    }
    Ok(())
}
