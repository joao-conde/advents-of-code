use std::collections::{HashSet, VecDeque};

fn main() {
    let input = std::fs::read_to_string("input/day10").unwrap();
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let (loop_tiles, p1) = find_loop_tiles(&grid);
    println!("Part1: {p1}");

    let p2: usize = grid
        .iter()
        .enumerate()
        .map(|(i, _)| count_enclosed_tiles(&grid, &loop_tiles, i))
        .sum();
    println!("Part2: {p2}");
}

fn find_loop_tiles(grid: &[Vec<char>]) -> (HashSet<(usize, usize)>, usize) {
    // tiles and the connections they can receive
    let north_conns = ['|', 'L', 'J', 'S'];
    let south_conns = ['|', '7', 'F', 'S'];
    let west_conns = ['-', 'J', '7', 'S'];
    let east_conns = ['-', 'L', 'F', 'S'];

    // grid dimensions and starting point
    let nrows = grid.len();
    let ncols = grid[0].len();
    let (si, sj) = grid
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|&c| c == 'S').map(|j| (i, j)))
        .unwrap();

    // initialize structures for breadth-first search
    // and keep track of maximum distance from start
    let mut max_steps = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut deque: VecDeque<(usize, usize, usize)> = VecDeque::new();
    deque.push_back((si, sj, 0));

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

    (visited, max_steps)
}

fn count_enclosed_tiles(
    grid: &[Vec<char>],
    loop_tiles: &HashSet<(usize, usize)>,
    i: usize,
) -> usize {
    let mut counts = 0;
    let mut inside = false;

    // raycast the entire row and keep track of loop
    // entrances and exits, counting enclosed tiles
    for (j, &tile) in grid[i].iter().enumerate() {
        if loop_tiles.contains(&(i, j)) {
            // we traverse the row horizontally so when
            // we cross a tile that represents a vertical
            // wall we enter or leave the loop
            inside = match tile {
                '|' | 'J' | 'L' => !inside,
                _ => inside,
            };
        } else if inside {
            counts += 1;
        }
    }

    counts
}
