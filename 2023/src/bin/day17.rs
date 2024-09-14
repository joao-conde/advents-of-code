use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

#[derive(Debug, Clone, Eq)]
struct State {
    i: usize,
    j: usize,
    direction: Direction,
    streak: usize,
    cost: u32,
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.i.hash(state);
        self.j.hash(state);
        self.direction.hash(state);
        self.streak.hash(state);
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i
            && self.j == other.j
            && self.direction == other.direction
            && self.streak == other.streak
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn forward(&self, map: &Vec<Vec<u32>>) -> Option<State> {
        let nrows = map.len();
        let ncols = map[0].len();

        let (di, dj) = self.direction.delta();

        let ni = self.i.checked_add_signed(di).filter(|i| *i < nrows)?;
        let nj = self.j.checked_add_signed(dj).filter(|j| *j < ncols)?;

        Some(Self {
            i: ni,
            j: nj,
            direction: self.direction,
            streak: self.streak + 1,
            cost: self.cost + map[ni][nj],
        })
    }

    fn turn_right(&self) -> State {
        Self {
            i: self.i,
            j: self.j,
            direction: self.direction.turn_right(),
            streak: 0,
            cost: self.cost,
        }
    }

    fn turn_left(&self) -> State {
        Self {
            i: self.i,
            j: self.j,
            direction: self.direction.turn_left(),
            streak: 0,
            cost: self.cost,
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input/day17").unwrap();

    let map: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().flat_map(|c| c.to_digit(10)).collect())
        .collect();

    let nrows = map.len();
    let ncols = map[0].len();

    let mut points: BinaryHeap<State> = BinaryHeap::new();
    let mut visited = HashSet::new();

    points.push(State {
        i: 0,
        j: 0,
        direction: Direction::Right,
        streak: 0,
        cost: 0,
    });

    while let Some(state) = points.pop() {
        if state.i == nrows - 1 && state.j == ncols - 1 && state.streak >= 4 {
            dbg!(&state);
            println!("found it with {}", state.cost);
            return;
        }

        if state.i >= nrows || state.j >= ncols {
            continue;
        }

        if visited.contains(&(state.i, state.j, state.direction, state.streak)) {
            continue;
        }
        visited.insert((state.i, state.j, state.direction, state.streak));

        if state.streak >= 4 {
            if let Some(state) = state.turn_right().forward(&map) {
                points.push(state);
            }

            if let Some(state) = state.turn_left().forward(&map) {
                points.push(state);
            }
        }

        // if we dont have to move yet, explore going forward
        if state.streak < 10 {
            if let Some(state) = state.forward(&map) {
                points.push(state);
            }
        }
    }
}
