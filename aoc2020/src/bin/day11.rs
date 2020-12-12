use std::fs;

type Grid = Vec<Vec<char>>;
type VisibleFn = fn(&Grid, usize, usize) -> Vec<&char>;

const OCCUPIED: char = '#';
const EMPTY: char = 'L';
const FLOOR: char = '.';

fn main() {
    let input = fs::read_to_string("input/day11").expect("failure opening input file");
    let seats = input.split('\n').map(|row| row.chars().collect()).collect::<Grid>();
    println!("Part1: {}", stabilize(seats.clone(), visible1, 4));
    println!("Part2: {}", stabilize(seats, visible2, 5));
}

fn stabilize(mut seats: Grid, visible_fn: VisibleFn, tolerance: usize) -> usize {
    let mut prev_count = seats.iter().flatten().filter(|c| **c == OCCUPIED).count();
    loop {
        seats = step(&seats, &visible_fn, tolerance);
        let occupied = seats.iter().flatten().filter(|c| **c == OCCUPIED).count();
        if occupied == prev_count {
            return prev_count;
        }
        prev_count = occupied;
    }
}

fn step(seats: &Grid, visible_fn: &VisibleFn, tolerance: usize) -> Grid {
    let mut next_seats = seats.to_owned();
    for i in 0..seats.len() {
        for j in 0..seats[i].len() {
            let occupied = visible_fn(seats, i, j).iter().filter(|c| ***c == OCCUPIED).count();
            match seats[i][j] {
                EMPTY if occupied == 0 => next_seats[i][j] = OCCUPIED,
                OCCUPIED if occupied >= tolerance => next_seats[i][j] = EMPTY,
                _ => (),
            }
        }
    }
    next_seats
}

fn visible1(seats: &Grid, i: usize, j: usize) -> Vec<&char> {
    [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
        .iter()
        .map(|(di, dj)| seats.get((i as i32 + di) as usize).and_then(|row| row.get((j as i32 + dj) as usize)).unwrap_or(&FLOOR))
        .collect()
}

fn visible2(seats: &Grid, i: usize, j: usize) -> Vec<&char> {
    [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
        .iter()
        .map(|(di, dj)| {
            let mut visible = vec![];
            let mut mult = 1;
            while let Some(c) = seats.get((i as i32 + di * mult) as usize).and_then(|row| row.get((j as i32 + dj * mult) as usize)) {
                visible.push(c);
                mult += 1;
                match *c {
                    OCCUPIED => break,
                    EMPTY => break,
                    _ => continue,
                }
            }
            visible
        })
        .flatten()
        .collect()
}
