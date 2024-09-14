type Point = (usize, usize);

fn main() {
    let input = std::fs::read_to_string("input/day11").unwrap();

    let universe: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let galaxies: Vec<Point> = universe
        .iter()
        .enumerate()
        .flat_map(|(i, r)| {
            r.iter()
                .enumerate()
                .map(|(j, _)| (i, j))
                .collect::<Vec<Point>>()
        })
        .filter(|&(i, j)| universe[i][j] == '#')
        .collect();

    let galaxy_pairs = unordered_pairs(&galaxies);

    let p1: usize = galaxy_pairs
        .iter()
        .map(|(g1, g2)| distance(&universe, 2, *g1, *g2))
        .sum();
    println!("Part1: {p1}");

    let p2: usize = galaxy_pairs
        .iter()
        .map(|(g1, g2)| distance(&universe, 1_000_000, *g1, *g2))
        .sum();
    println!("Part2: {p2}");
}

fn distance(universe: &Vec<Vec<char>>, expansion: usize, g1: Point, g2: Point) -> usize {
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

fn unordered_pairs(points: &[Point]) -> Vec<(Point, Point)> {
    let pairs: Vec<(Point, Point)> = points
        .iter()
        .flat_map(|g1| points.iter().map(|g2| (*g1, *g2)))
        .filter(|(g1, g2)| g1 != g2)
        .collect();

    let mut unique = vec![];
    for (g1, g2) in pairs {
        if !unique.contains(&(g2, g1)) {
            unique.push((g1, g2))
        }
    }

    unique
}
