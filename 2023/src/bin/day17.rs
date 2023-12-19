use std::collections::{BinaryHeap, HashSet};

fn main() {
    let input = std::fs::read_to_string("input/day17").unwrap();

    let map: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().flat_map(|c| c.to_digit(10)).collect())
        .collect();

    let nrows = map.len();
    let ncols = map[0].len();

    let mut points: BinaryHeap<(u32, usize, usize, usize, char, String)> = BinaryHeap::new();
    let mut visited = HashSet::new();

    points.push((0, 0, 0, 1, 'r', "r".to_owned()));
    points.push((0, 0, 0, 1, 'd', "d".to_owned()));
    while let Some((cost, i, j, distance, dir, dirs)) = points.pop() {
        if i == nrows - 1 && j == ncols - 1 {
            println!("found it with {cost} {dirs}");
            return;
        }

        if visited.contains(&(i, j, cost, distance)) {
            continue;
        }

        visited.insert((i, j, cost, distance));

        let cost = cost + map[i][j];

        if i < nrows - 1 && (dir != 'd' || distance != 3) {
            let distance = if dir != 'd' { 1 } else { distance + 1 };
            points.push((cost, i + 1, j, distance, 'd', format!("{dirs}d")));
        }

        if j < ncols - 1 && (dir != 'r' || distance != 3) {
            let distance = if dir != 'r' { 1 } else { distance + 1 };
            points.push((cost, i, j + 1, distance, 'r', format!("{dirs}r")));
        }

        if i > 0 && (dir != 'u' || distance != 3) {
            let distance = if dir != 'u' { 1 } else { distance + 1 };
            points.push((cost, i - 1, j, distance, 'u', format!("{dirs}u")));
        }
    }
}
