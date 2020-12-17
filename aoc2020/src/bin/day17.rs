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

fn step_3D(grid: &Grid3D) -> Grid3D {
    let mut next = grid.clone();

    let dim = *grid.keys().map(|(x, y, z)| *[x, y, z].iter().max().unwrap()).max().unwrap();

    for x in -dim..dim + 2 {
        for y in -dim..dim + 2 {
            for z in -dim..dim + 2 {
                let mut active = 0;
                for dx in -1..2 {
                    for dy in -1..2 {
                        for dz in -1..2 {
                            if dx == 0 && dy == 0 && dz == 0 {
                                continue;
                            }
                            let x = x + dx;
                            let y = y + dy;
                            let z = z + dz;
                            active += if *grid.get(&(x, y, z)).unwrap_or(&'.') == '#' { 1 } else { 0 };
                        }
                    }
                }
                match grid.get(&(x, y, z)).unwrap_or(&'.') {
                    '#' if active != 2 && active != 3 => next.insert((x, y, z), '.'),
                    '.' if active == 3 => next.insert((x, y, z), '#'),
                    _ => None,
                };
            }
        }
    }

    next
}

fn step_4D(grid: &Grid4D) -> Grid4D {
    let mut next = grid.clone();

    let dim = *grid.keys().map(|(x, y, z, w)| *[x, y, z, w].iter().max().unwrap()).max().unwrap();

    for x in -dim..dim + 2 {
        for y in -dim..dim + 2 {
            for z in -dim..dim + 2 {
                for w in -dim..dim + 2 {
                    let mut active = 0;
                    for dx in -1..2 {
                        for dy in -1..2 {
                            for dz in -1..2 {
                                for dw in -1..2 {
                                    if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                                        continue;
                                    }
                                    let x = x + dx;
                                    let y = y + dy;
                                    let z = z + dz;
                                    let w = w + dw;
                                    active += if *grid.get(&(x, y, z, w)).unwrap_or(&'.') == '#' { 1 } else { 0 };
                                }
                            }
                        }
                    }
                    match grid.get(&(x, y, z, w)).unwrap_or(&'.') {
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
