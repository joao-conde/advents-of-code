use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(PartialEq)]
enum Tile {
    Empty,
    MirrorRight,
    MirrorLeft,
    SplitVertical,
    SplitHorizontal,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '/' => Tile::MirrorRight,
            '\\' => Tile::MirrorLeft,
            '|' => Tile::SplitVertical,
            '-' => Tile::SplitHorizontal,
            _ => unreachable!(),
        }
    }
}

impl Direction {
    fn is_horizontal(&self) -> bool {
        match self {
            Direction::Right | Direction::Left => true,
            Direction::Up | Direction::Down => false,
        }
    }

    fn is_vertical(&self) -> bool {
        !self.is_horizontal()
    }
}

fn main() {
    let input = std::fs::read_to_string("input/day16").unwrap();

    let grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.into()).collect())
        .collect();

    let p1 = energized(&grid, (0, 0, Direction::Right));
    println!("Part1: {p1}");

    let p2 = build_starts(&grid)
        .iter()
        .map(|start| energized(&grid, *start))
        .max()
        .unwrap();
    println!("Part2: {p2}");
}

fn energized(grid: &[Vec<Tile>], start: (isize, isize, Direction)) -> usize {
    let nrows = grid.len() as isize;
    let ncols = grid[0].len() as isize;

    let mut beams: Vec<(isize, isize, Direction)> = Vec::new();
    let mut beams_seen: HashSet<(isize, isize, Direction)> = HashSet::new();
    let mut energized: HashSet<(isize, isize)> = HashSet::new();

    beams.push(start);

    while let Some((mut i, mut j, mut dir)) = beams.pop() {
        while i >= 0 && i < nrows && j >= 0 && j < ncols && !beams_seen.contains(&(i, j, dir)) {
            energized.insert((i, j));
            beams_seen.insert((i, j, dir));

            let tile = &grid[i as usize][j as usize];

            if tile == &Tile::MirrorLeft {
                // '\'
                dir = match dir {
                    Direction::Right => Direction::Down,
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                }
            } else if tile == &Tile::MirrorRight {
                // '/'
                dir = match dir {
                    Direction::Right => Direction::Up,
                    Direction::Left => Direction::Down,
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                }
            } else if dir.is_horizontal() && tile == &Tile::SplitVertical {
                beams.push((i - 1, j, Direction::Up));
                beams.push((i + 1, j, Direction::Down));
                break;
            } else if dir.is_vertical() && tile == &Tile::SplitHorizontal {
                beams.push((i, j - 1, Direction::Left));
                beams.push((i, j + 1, Direction::Right));
                break;
            }

            if dir == Direction::Up {
                i -= 1;
            } else if dir == Direction::Down {
                i += 1;
            } else if dir == Direction::Left {
                j -= 1;
            } else if dir == Direction::Right {
                j += 1;
            }
        }
    }

    energized.len()
}

fn build_starts(grid: &[Vec<Tile>]) -> Vec<(isize, isize, Direction)> {
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
