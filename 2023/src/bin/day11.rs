fn main() {
    let input = std::fs::read_to_string("input/day11").unwrap();

    let universe: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let galaxies: Vec<(usize, usize)> = universe
        .iter()
        .enumerate()
        .flat_map(|(i, r)| {
            r.iter()
                .enumerate()
                .map(|(j, _)| (i, j))
                .collect::<Vec<(usize, usize)>>()
        })
        .filter(|&(i, j)| universe[i][j] == '#')
        .collect();

    let pairs: Vec<((usize, usize), (usize, usize))> = galaxies
        .iter()
        .flat_map(|g1| galaxies.iter().map(|g2| (*g1, *g2)))
        .filter(|(g1, g2)| g1 != g2)
        .collect();

    let mut unique = vec![];
    for (g1, g2) in pairs {
        if !unique.contains(&(g2, g1)) {
            unique.push((g1, g2))
        }
    }

    let sum: usize = unique
        .iter()
        .map(|(g1, g2)| distance(&universe, 2, *g1, *g2))
        .sum();
    dbg!(sum);

    let sum: usize = unique
        .iter()
        .map(|(g1, g2)| distance(&universe, 1_000_000, *g1, *g2))
        .sum();
    dbg!(sum);
}

fn distance(
    universe: &Vec<Vec<char>>,
    expansion: usize,
    g1: (usize, usize),
    g2: (usize, usize),
) -> usize {
    let nrows = universe.len();
    let ncols = universe[0].len();

    let min_i = usize::min(g1.0, g2.0);
    let max_i = usize::max(g1.0, g2.0);
    let min_j = usize::min(g1.1, g2.1);
    let max_j = usize::max(g1.1, g2.1);

    let empty_rows = (min_i..max_i)
        .filter(|i| (0..ncols).all(|j| universe[*i][j] == '.'))
        .count();

    let empty_cols = (min_j..max_j)
        .filter(|j| (0..nrows).all(|i| universe[i][*j] == '.'))
        .count();

    (max_j - min_j)
        + (max_i - min_i)
        + (empty_rows * (expansion - 1))
        + (empty_cols * (expansion - 1))
}
