fn main() {
    let input = std::fs::read_to_string("input/day18").unwrap();
    let p1 = lagoon_capacity(&build_edge(&p1_plan(&input)));
    let p2 = lagoon_capacity(&build_edge(&p2_plan(&input)));
    println!("Part1: {p1}");
    println!("Part2: {p2}");
}

fn p1_plan(input: &str) -> Vec<(char, usize)> {
    input
        .lines()
        .map(|l| {
            let mut step = l.splitn(3, ' ');
            let dir = step.next().unwrap().parse().unwrap();
            let meters = step.next().unwrap().parse().unwrap();
            (dir, meters)
        })
        .collect()
}

fn p2_plan(input: &str) -> Vec<(char, usize)> {
    input
        .lines()
        .map(|l| {
            let (_, hex) = l.split_once("(#").unwrap();
            let meters = usize::from_str_radix(&hex[..5], 16).unwrap();
            let dir = match usize::from_str_radix(&hex[5..=5], 16).unwrap() {
                0 => 'R',
                1 => 'D',
                2 => 'L',
                3 => 'U',
                _ => unreachable!(),
            };
            (dir, meters)
        })
        .collect()
}

fn build_edge(plan: &[(char, usize)]) -> Vec<(isize, isize)> {
    let mut edge: Vec<(isize, isize)> = Vec::from_iter([(0, 0)]);

    let (mut i, mut j) = (0, 0);
    for (dir, meters) in plan {
        let (di, dj) = match dir {
            'U' => (-1, 0),
            'D' => (1, 0),
            'L' => (0, -1),
            'R' => (0, 1),
            _ => unreachable!(),
        };

        for _ in 1..=*meters {
            i += di;
            j += dj;
            edge.push((i, j));
        }
    }

    edge
}

fn lagoon_capacity(edge: &[(isize, isize)]) -> usize {
    edge.len() + pick_theorem(edge)
}

fn pick_theorem(edge: &[(isize, isize)]) -> usize {
    shoelace_area(edge) - (edge.len() / 2)
}

fn shoelace_area(edge: &[(isize, isize)]) -> usize {
    let edge_len = edge.len();
    (0..edge_len)
        .map(|i| {
            let (x1, y1) = edge[i];
            let (x2, y2) = edge[(i + 1) % edge_len];
            x1 * y2 - x2 * y1
        })
        .sum::<isize>()
        .unsigned_abs()
        / 2
}
