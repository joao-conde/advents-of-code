use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::hash::Hash;

// State representing a node in the graph of different possible paths.
// Used in the Dijkstra's algorithm. States are considered equal by comparing
// every field but the cost.
// States are ordered with the cost however.
#[derive(Clone, Copy, Eq)]
struct State {
    i: usize,
    j: usize,
    direction: Direction,
    moves: usize,
    cost: u32,
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

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i
            && self.j == other.j
            && self.direction == other.direction
            && self.moves == other.moves
    }
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.i.hash(state);
        self.j.hash(state);
        self.direction.hash(state);
        self.moves.hash(state);
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
            moves: self.moves + 1,
            cost: self.cost + map[ni][nj],
        })
    }

    fn turn_right(&self) -> State {
        Self {
            i: self.i,
            j: self.j,
            direction: self.direction.turn_right(),
            moves: 0,
            cost: self.cost,
        }
    }

    fn turn_left(&self) -> State {
        Self {
            i: self.i,
            j: self.j,
            direction: self.direction.turn_left(),
            moves: 0,
            cost: self.cost,
        }
    }
}

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

fn main() {
    let input = std::fs::read_to_string("input/day17").unwrap();

    let map: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().flat_map(|c| c.to_digit(10)).collect())
        .collect();

    let p1 = heat_loss(&map, 0, 3);
    println!("Part1: {p1}");

    let p2 = heat_loss(&map, 4, 10);
    println!("Part2: {p2}");
}

// Minimum heat loss computation using Dijkstra's algorithm.
fn heat_loss(map: &Vec<Vec<u32>>, min_moves: usize, max_moves: usize) -> u32 {
    let nrows = map.len();
    let ncols = map[0].len();

    let mut visited = HashSet::new();
    let mut states = BinaryHeap::new();
    states.push(State {
        i: 0,
        j: 0,
        direction: Direction::Right,
        moves: 0,
        cost: 0,
    });

    while let Some(state) = states.pop() {
        // if we have reached the bottom-right grid position with our
        // required minimum number of moves we found the minimum cost
        if state.i == nrows - 1 && state.j == ncols - 1 && state.moves >= min_moves {
            return state.cost;
        }

        // if we have seen this state (same position, direction and moves)
        // we skip expanding this path, otherwise mark it as seen
        if visited.contains(&state) {
            continue;
        }
        visited.insert(state);

        // if we are in a state that has made the minimum number of moves
        // this means we can afford to try to turn left or right
        // we dont turn if out of bounds
        if state.moves >= min_moves {
            if let Some(state) = state.turn_right().forward(map) {
                states.push(state);
            }

            if let Some(state) = state.turn_left().forward(map) {
                states.push(state);
            }
        }

        // if we are in a state that has not yet reached the maximum number
        // of moves we can afford to move forward
        if state.moves < max_moves {
            if let Some(state) = state.forward(map) {
                states.push(state);
            }
        }
    }

    unreachable!()
}
