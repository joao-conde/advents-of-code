use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

type Grid3D = HashMap<(i32, i32, i32), char>;
type Grid4D = HashMap<(i32, i32, i32, i32), char>;

fn main() {
    let input = fs::read_to_string("input/day17").expect("failure opening input file");

    let mut grid3D = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, state)| {
            grid3D.insert((x as i32, y as i32, 0), state);
        })
    });

    let mut grid4D = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, state)| {
            grid4D.insert((x as i32, y as i32, 0, 0), state);
        })
    });

    (0..6).for_each(|_| {
        grid3D = step_3D(&grid3D);
        grid4D = step_4D(&grid4D);
    });
    println!("Part1: {}", grid3D.values().filter(|c| **c == '#').count());
    println!("Part2: {}", grid4D.values().filter(|c| **c == '#').count());
}

fn step_3D(prev: &Grid3D) -> Grid3D {
    let mut next = prev.clone();
    let dim = *prev.keys().map(|(x, y, z)| *[x, y, z].iter().max().unwrap()).max().unwrap();
    let deltas = (-1..2).cartesian_product((-1..2).cartesian_product(-1..2).collect_vec());
    let deltas = deltas.filter(|(x, (y, z))| !(*x == 0 && *y == 0 && *z == 0)).collect_vec();
    for x in -dim..dim + 2 {
        for y in -dim..dim + 2 {
            for z in -dim..dim + 2 {
                let active = deltas
                    .iter()
                    .map(|(dx, (dy, dz))| if *prev.get(&(x + dx, y + dy, z + dz)).unwrap_or(&'.') == '#' { 1 } else { 0 })
                    .sum::<i32>();
                match prev.get(&(x, y, z)).unwrap_or(&'.') {
                    '#' if active != 2 && active != 3 => next.insert((x, y, z), '.'),
                    '.' if active == 3 => next.insert((x, y, z), '#'),
                    _ => None,
                };
            }
        }
    }
    next
}

fn step_4D(prev: &Grid4D) -> Grid4D {
    let mut next = prev.clone();
    let dim = *prev.keys().map(|(x, y, z, w)| *[x, y, z, w].iter().max().unwrap()).max().unwrap();
    let deltas = (-1..2).cartesian_product((-1..2).cartesian_product((-1..2).cartesian_product(-1..2)));
    let deltas = deltas.filter(|(x, (y, (z, w)))| !(*x == 0 && *y == 0 && *z == 0 && *w == 0)).collect_vec();
    for x in -dim..dim + 2 {
        for y in -dim..dim + 2 {
            for z in -dim..dim + 2 {
                for w in -dim..dim + 2 {
                    let active = deltas
                        .iter()
                        .map(|(dx, (dy, (dz, dw)))| if *prev.get(&(x + dx, y + dy, z + dz, w + dw)).unwrap_or(&'.') == '#' { 1 } else { 0 })
                        .sum::<i32>();
                    match prev.get(&(x, y, z, w)).unwrap_or(&'.') {
                        '#' if active != 2 && active != 3 => next.insert((x, y, z, w), '.'),
                        '.' if active == 3 => next.insert((x, y, z, w), '#'),
                        _ => None,
                    };
                }
            }
        }
    }
    next
}
