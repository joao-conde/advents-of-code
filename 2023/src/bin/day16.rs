use std::collections::HashSet;

type Beam = (isize, isize, Direction);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn is_horizontal(&self) -> bool {
        match self {
            Direction::Up | Direction::Down => false,
            Direction::Right | Direction::Left => true,
        }
    }

    fn is_vertical(&self) -> bool {
        !self.is_horizontal()
    }

    fn mirrored_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn mirrored_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input/day16").unwrap();

    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let p1 = energized(&grid, (0, 0, Direction::Right));
    println!("Part1: {p1}");

    let p2 = build_starts(&grid)
        .iter()
        .map(|start| energized(&grid, *start))
        .max()
        .unwrap();
    println!("Part2: {p2}");
}

fn energized(grid: &[Vec<char>], start: Beam) -> usize {
    let nrows = grid.len() as isize;
    let ncols = grid[0].len() as isize;

    let mut beams = vec![start];
    let mut beams_seen: HashSet<Beam> = HashSet::new();
    let mut energized: HashSet<(isize, isize)> = HashSet::new();

    while let Some((mut i, mut j, mut dir)) = beams.pop() {
        while i >= 0 && i < nrows && j >= 0 && j < ncols && !beams_seen.contains(&(i, j, dir)) {
            beams_seen.insert((i, j, dir));
            energized.insert((i, j));

            match grid[i as usize][j as usize] {
                '\\' => dir = dir.mirrored_left(),
                '/' => dir = dir.mirrored_right(),
                '|' if dir.is_horizontal() => {
                    beams.push((i - 1, j, Direction::Up));
                    beams.push((i + 1, j, Direction::Down));
                    break;
                }
                '-' if dir.is_vertical() => {
                    beams.push((i, j - 1, Direction::Left));
                    beams.push((i, j + 1, Direction::Right));
                    break;
                }
                _ => (),
            };

            match dir {
                Direction::Up => i -= 1,
                Direction::Down => i += 1,
                Direction::Left => j -= 1,
                Direction::Right => j += 1,
            };
        }
    }

    energized.len()
}

fn build_starts(grid: &[Vec<char>]) -> Vec<Beam> {
    let nrows = grid.len() as isize;
    let ncols = grid[0].len() as isize;
    let mut starts = vec![];

    for (i, _) in grid.iter().enumerate() {
        starts.push((i as isize, 0, Direction::Right));
        starts.push((i as isize, ncols - 1, Direction::Left));
    }

    for (j, _) in grid[0].iter().enumerate() {
        starts.push((0, j as isize, Direction::Down));
        starts.push((nrows - 1, j as isize, Direction::Up));
    }

    starts
}
