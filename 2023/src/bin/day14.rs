use std::{collections::HashSet, str::FromStr, vec};

#[derive(Clone, PartialEq, Eq, Hash)]
struct Platform {
    rocks: Vec<Vec<char>>,
    cubes: Vec<(usize, usize)>,
    nrows: usize,
    ncols: usize,
}

impl Platform {
    fn load(&self) -> usize {
        self.rocks
            .iter()
            .skip(1)
            .rev()
            .skip(1)
            .rev()
            .enumerate()
            .map(|(i, row)| row.iter().filter(|c| **c == 'O').count() * (self.nrows - 2 - i))
            .sum()
    }

    fn spin(&mut self) {
        self.slide_north();
        self.slide_west();
        self.slide_south();
        self.slide_east();
    }

    fn slide_north(&mut self) {
        let previous = self.rocks.clone();

        for (ci, cj) in &self.cubes {
            let rocks_below = (ci + 1..self.nrows)
                .take_while(|i| previous[*i][*cj] != '#')
                .filter(|i| previous[*i][*cj] == 'O')
                .count();

            (ci + 1..=ci + rocks_below).for_each(|i| self.rocks[i][*cj] = 'O');
            (ci + rocks_below + 1..self.nrows)
                .take_while(|i| previous[*i][*cj] != '#')
                .for_each(|i| self.rocks[i][*cj] = '.');
        }
    }

    fn slide_west(&mut self) {
        let previous = self.rocks.clone();

        for (ci, cj) in &self.cubes {
            let rocks_right = (cj + 1..self.ncols)
                .take_while(|j| previous[*ci][*j] != '#')
                .filter(|j| previous[*ci][*j] == 'O')
                .count();

            (cj + 1..=cj + rocks_right).for_each(|j| self.rocks[*ci][j] = 'O');
            (cj + rocks_right + 1..self.ncols)
                .take_while(|j| previous[*ci][*j] != '#')
                .for_each(|j| self.rocks[*ci][j] = '.');
        }
    }

    fn slide_south(&mut self) {
        let previous = self.rocks.clone();

        for (ci, cj) in &self.cubes {
            let to = ci.saturating_sub(1);
            let rocks_above = (0..=to)
                .rev()
                .take_while(|i| previous[*i][*cj] != '#')
                .filter(|i| previous[*i][*cj] == 'O')
                .count();

            if rocks_above > 0 {
                (ci - rocks_above..*ci).for_each(|i| self.rocks[i][*cj] = 'O');
                (0..ci - rocks_above)
                    .rev()
                    .take_while(|i| previous[*i][*cj] != '#')
                    .for_each(|i| self.rocks[i][*cj] = '.');
            }
        }
    }

    fn slide_east(&mut self) {
        let previous = self.rocks.clone();

        for (ci, cj) in &self.cubes {
            let to = cj.saturating_sub(1);

            let rocks_left = (0..=to)
                .rev()
                .take_while(|j| previous[*ci][*j] != '#')
                .filter(|j| previous[*ci][*j] == 'O')
                .count();

            if rocks_left > 0 {
                (cj - rocks_left..*cj).for_each(|j| self.rocks[*ci][j] = 'O');
                (0..cj - rocks_left)
                    .rev()
                    .take_while(|j| previous[*ci][*j] != '#')
                    .for_each(|j| self.rocks[*ci][j] = '.');
            }
        }
    }
}

impl FromStr for Platform {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rocks: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();

        let nrows = rocks.len();
        let ncols = rocks[0].len();

        // add a rock padding to the platform that
        // prevents the rocks from "falling"
        rocks.insert(0, vec!['#'; ncols]);
        rocks.insert(nrows + 1, vec!['#'; ncols + 2]);
        (0..nrows + 1).for_each(|i| rocks[i].insert(0, '#'));
        (0..nrows + 1).for_each(|i| rocks[i].insert(ncols + 1, '#'));

        let nrows = rocks.len();
        let ncols = rocks[0].len();

        let cubes = rocks
            .iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, _)| (i, j)))
            .filter(|(i, j)| rocks[*i][*j] == '#')
            .collect();

        Ok(Self {
            rocks,
            cubes,
            nrows,
            ncols,
        })
    }
}

fn main() {
    let input = std::fs::read_to_string("input/day14").unwrap();

    let mut platform = Platform::from_str(&input).unwrap();

    let mut seen = HashSet::new();

    let mut states = vec![];

    for _ in 0..1_000_000_000 {
        let state = platform.clone();
        if seen.contains(&state) {
            break;
        }
        seen.insert(state.clone());
        states.push(state);

        platform.spin();
    }

    let first_repeat = states.iter().position(|s| *s == platform).unwrap();

    let state_i = (1_000_000_000 - first_repeat) % (states.len() - first_repeat) + first_repeat;

    println!("{}", states[state_i].load());
}
