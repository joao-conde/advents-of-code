use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input/day18").unwrap();

    let plan: Vec<(char, usize, &str)> = input
        .lines()
        .map(|l| {
            let mut step = l.splitn(3, ' ');
            let dir = step.next().unwrap().parse().unwrap();
            let meters = step.next().unwrap().parse().unwrap();
            let color = step.next().unwrap();
            (dir, meters, color)
        })
        .collect();

    let mut edge: HashSet<(isize, isize)> = HashSet::from_iter([(0, 0)]);

    let (mut i, mut j) = (0, 0);
    for (dir, meters, _) in plan {
        let (di, dj) = match dir {
            'U' => (-1, 0),
            'D' => (1, 0),
            'L' => (0, -1),
            'R' => (0, 1),
            _ => unreachable!(),
        };

        for _ in 1..=meters {
            i += di;
            j += dj;
            edge.insert((i, j));
        }
    }

    dbg!(edge.len());
}
