use std::str::FromStr;

#[derive(Clone, PartialEq)]
struct Platform {
    rocks: Vec<Vec<Tile>>,
    cube_positions: Vec<Point>,
    nrows: usize,
    ncols: usize,
}

impl Platform {
    fn total_load(&self) -> usize {
        let unpadded_rocks = self.rocks.iter().skip(1).rev().skip(1).rev();
        let unpadded_nrows = self.nrows - 2;
        unpadded_rocks
            .enumerate()
            .map(|(i, row)| {
                row.iter().filter(|c| **c == Tile::Round).count() * (unpadded_nrows - i)
            })
            .sum()
    }

    fn spin(&mut self) {
        self.slide_north();
        self.slide_west();
        self.slide_south();
        self.slide_east();
    }

    fn slide_north(&mut self) {
        let rocks = self.rocks.clone();

        for cube in &self.cube_positions {
            let rocks_below = (cube.i + 1..self.nrows)
                .take_while(|i| rocks[*i][cube.j] != Tile::Cube)
                .filter(|i| rocks[*i][cube.j] == Tile::Round)
                .count();

            (cube.i + 1..=cube.i + rocks_below).for_each(|i| self.rocks[i][cube.j] = Tile::Round);
            (cube.i + rocks_below + 1..self.nrows)
                .take_while(|i| rocks[*i][cube.j] != Tile::Cube)
                .for_each(|i| self.rocks[i][cube.j] = Tile::Empty);
        }
    }

    fn slide_west(&mut self) {
        let rocks = self.rocks.clone();

        for cube in &self.cube_positions {
            let rocks_right = (cube.j + 1..self.ncols)
                .take_while(|j| rocks[cube.i][*j] != Tile::Cube)
                .filter(|j| rocks[cube.i][*j] == Tile::Round)
                .count();

            (cube.j + 1..=cube.j + rocks_right).for_each(|j| self.rocks[cube.i][j] = Tile::Round);
            (cube.j + rocks_right + 1..self.ncols)
                .take_while(|j| rocks[cube.i][*j] != Tile::Cube)
                .for_each(|j| self.rocks[cube.i][j] = Tile::Empty);
        }
    }

    fn slide_south(&mut self) {
        let rocks = self.rocks.clone();

        for cube in &self.cube_positions {
            let to = cube.i.saturating_sub(1);
            let rocks_above = (0..=to)
                .rev()
                .take_while(|i| rocks[*i][cube.j] != Tile::Cube)
                .filter(|i| rocks[*i][cube.j] == Tile::Round)
                .count();

            (cube.i - rocks_above..cube.i).for_each(|i| self.rocks[i][cube.j] = Tile::Round);
            (0..cube.i - rocks_above)
                .rev()
                .take_while(|i| rocks[*i][cube.j] != Tile::Cube)
                .for_each(|i| self.rocks[i][cube.j] = Tile::Empty);
        }
    }

    fn slide_east(&mut self) {
        let rocks = self.rocks.clone();

        for cube in &self.cube_positions {
            let to = cube.j.saturating_sub(1);
            let rocks_left = (0..=to)
                .rev()
                .take_while(|j| rocks[cube.i][*j] != Tile::Cube)
                .filter(|j| rocks[cube.i][*j] == Tile::Round)
                .count();

            (cube.j - rocks_left..cube.j).for_each(|j| self.rocks[cube.i][j] = Tile::Round);
            (0..cube.j - rocks_left)
                .rev()
                .take_while(|j| rocks[cube.i][*j] != Tile::Cube)
                .for_each(|j| self.rocks[cube.i][j] = Tile::Empty);
        }
    }
}

impl FromStr for Platform {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rocks: Vec<Vec<Tile>> = s
            .lines()
            .map(|l| l.chars().map(Tile::from).collect())
            .collect();

        let nrows = rocks.len();
        let ncols = rocks[0].len();

        // add a rock padding to the platform that
        // prevents the rocks from "falling"
        rocks.insert(0, vec![Tile::Cube; ncols]);
        rocks.insert(nrows + 1, vec![Tile::Cube; ncols + 2]);
        (0..nrows + 1).for_each(|i| rocks[i].insert(0, Tile::Cube));
        (0..nrows + 1).for_each(|i| rocks[i].insert(ncols + 1, Tile::Cube));

        let nrows = rocks.len();
        let ncols = rocks[0].len();

        let cubes = rocks
            .iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, _)| Point { i, j }))
            .filter(|cube| rocks[cube.i][cube.j] == Tile::Cube)
            .collect();

        Ok(Self {
            rocks,
            cube_positions: cubes,
            nrows,
            ncols,
        })
    }
}

#[derive(Clone, PartialEq)]
struct Point {
    i: usize,
    j: usize,
}

#[derive(Clone, PartialEq)]
enum Tile {
    Round,
    Cube,
    Empty,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Cube,
            'O' => Self::Round,
            '.' => Self::Empty,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input/day14").unwrap();

    let p1 = p1(&input);
    println!("Part1: {p1}");

    let p2 = p2(&input);
    println!("Part2: {p2}");
}

fn p1(input: &str) -> usize {
    let mut platform = Platform::from_str(input).unwrap();
    platform.slide_north();
    platform.total_load()
}

fn p2(input: &str) -> usize {
    let mut platform = Platform::from_str(input).unwrap();

    // figure out when a state repeats after a cycle
    // which will lead to the same values from here
    // onwards i.e. there is a pattern
    let mut states = Vec::new();
    while !states.contains(&platform) {
        states.push(platform.clone());
        platform.spin();
    }

    // the index where the pattern first repeats
    // everything before this was not part of
    // the pattern yet
    let pattern_start = states.iter().position(|s| *s == platform).unwrap();
    let pattern_length = states.len() - pattern_start;

    // the number of iterations where the pattern
    // actually repeats
    let pattern_iterations = 1_000_000_000 - pattern_start;

    // figure out where in our pattern the last iteration ends
    let last_iteration_pattern_index = pattern_iterations % pattern_length;

    // adding the offset for the pattern to start with the final
    // pattern index gives us the state index to look for
    let last_state_index = pattern_start + last_iteration_pattern_index;
    let last_state = &states[last_state_index];

    last_state.total_load()
}
