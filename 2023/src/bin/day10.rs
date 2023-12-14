use std::collections::{HashSet, VecDeque};

fn main() {
    let input = std::fs::read_to_string("input/day10").unwrap();

    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let nrows = grid.len();
    let ncols = grid[0].len();

    let north_conns = ['|', 'L', 'J', 'S'];
    let south_conns = ['|', '7', 'F', 'S'];
    let west_conns = ['-', 'J', '7', 'S'];
    let east_conns = ['-', 'L', 'F', 'S'];

    let (si, sj) = grid
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|&c| c == 'S').map(|j| (i, j)))
        .unwrap();

    let mut deque: VecDeque<(usize, usize, usize)> = VecDeque::new();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    deque.push_back((si, sj, 0));

    let mut max_steps = 0;
    while let Some((i, j, distance)) = deque.pop_front() {
        if visited.contains(&(i, j)) {
            continue;
        }

        visited.insert((i, j));
        max_steps = max_steps.max(distance);

        let cur_tile = grid[i][j];

        if i > 0 && north_conns.contains(&cur_tile) && south_conns.contains(&grid[i - 1][j]) {
            deque.push_back((i - 1, j, distance + 1));
        }

        if i + 1 < nrows && south_conns.contains(&cur_tile) && north_conns.contains(&grid[i + 1][j])
        {
            deque.push_back((i + 1, j, distance + 1));
        }

        if j > 0 && west_conns.contains(&cur_tile) && east_conns.contains(&grid[i][j - 1]) {
            deque.push_back((i, j - 1, distance + 1));
        }

        if j + 1 < ncols && east_conns.contains(&cur_tile) && west_conns.contains(&grid[i][j + 1]) {
            deque.push_back((i, j + 1, distance + 1));
        }
    }

    println!("{max_steps}");
}
