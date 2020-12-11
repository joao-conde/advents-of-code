use std::fs;

type Grid = Vec<Vec<char>>;
type NeighborsFn = dyn Fn(&Grid, usize, usize) -> Vec<&char>;
type StepFn = dyn Fn(&Grid, &Box<NeighborsFn>, usize) -> Grid;

const OCCUPIED: char = '#';
const EMPTY: char = 'L';
const FLOOR: char = '.';

fn main() {
    let input = fs::read_to_string("input/day11").expect("failure opening input file");
    let seats = input.split('\n').map(|row| row.chars().collect()).collect::<Grid>();
    println!("Part1: {}", stabilize(seats.clone(), Box::new(step), Box::new(neighbors1), 4));
    println!("Part2: {}", stabilize(seats, Box::new(step), Box::new(neighbors2), 5));
}

fn stabilize(mut seats: Grid, step_fn: Box<StepFn>, neighbors_fn: Box<NeighborsFn>, occupation_threshold: usize) -> usize {
    let mut prev_count = seats.iter().flatten().filter(|c| **c == OCCUPIED).count();
    loop {
        seats = step_fn(&mut seats, &neighbors_fn, occupation_threshold);
        let occupied = seats.iter().flatten().filter(|c| **c == OCCUPIED).count();
        if occupied == prev_count {
            return prev_count;
        } else {
            prev_count = occupied;
        }
    }
}

fn step(seats: &Grid, neighbors_fn: &Box<NeighborsFn>, occupation_threshold: usize) -> Grid {
    let mut next_seats = seats.clone();
    for i in 0..seats.len() {
        for j in 0..seats[i].len() {
            let neighbors = neighbors_fn(seats, i, j);
            let occupied = neighbors.iter().filter(|c| ***c == OCCUPIED).count();
            match seats[i][j] {
                EMPTY if occupied == 0 => next_seats[i][j] = OCCUPIED,
                OCCUPIED if occupied >= occupation_threshold => next_seats[i][j] = EMPTY,
                _ => (),
            }
        }
    }
    next_seats
}

fn neighbors1(seats: &Grid, i: usize, j: usize) -> Vec<&char> {
    [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
        .iter()
        .map(|(di, dj)| seats.get((i as i32 + di) as usize).and_then(|row| row.get((j as i32 + dj) as usize)).unwrap_or(&FLOOR))
        .collect()
}

fn neighbors2(seats: &Grid, i: usize, j: usize) -> Vec<&char> {
    [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
        .iter()
        .map(|(di, dj)| {
            let mut chars = vec![];
            let mut mult = 1;
            while let Some(c) = seats.get((i as i32 + di * mult) as usize).and_then(|row| row.get((j as i32 + dj * mult) as usize)) {
                chars.push(c);
                mult += 1;
                match *c {
                    OCCUPIED => break,
                    EMPTY => break,
                    _ => continue,
                }
            }
            chars
        })
        .flatten()
        .collect()
}
