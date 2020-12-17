use itertools::Itertools;
use std::collections::HashSet;
use std::fs;
use std::hash::Hash;

type Grid3D = HashSet<[i32; 3]>;
type Grid4D = HashSet<[i32; 4]>;

fn main() {
    let input = fs::read_to_string("input/day17").expect("failure opening input file");

    let mut grid3D = input
        .lines()
        .enumerate()
        .map(|(y, line)| line.chars().enumerate().filter(|(_, cell)| *cell == '#').map(|(x, _)| [x as i32, y as i32, 0]).collect_vec())
        .flatten()
        .collect::<Grid3D>();

    let mut grid4D = input
        .lines()
        .enumerate()
        .map(|(y, line)| line.chars().enumerate().filter(|(_, cell)| *cell == '#').map(|(x, _)| [x as i32, y as i32, 0, 0]).collect_vec())
        .flatten()
        .collect::<Grid4D>();

    (0..6).for_each(|_| {
        grid3D = step_3D(&grid3D);
        grid4D = step_4D(&grid4D);
    });
    println!("Part1: {}", grid3D.len());
    println!("Part2: {}", grid4D.len());
}

fn step_3D(prev: &Grid3D) -> Grid3D {
    let mut next = prev.clone();
    let dim = *prev.iter().map(|[x, y, z]| *[x, y, z].iter().max().unwrap()).max().unwrap();
    let deltas = (-1..2).cartesian_product((-1..2).cartesian_product(-1..2).collect_vec());
    let deltas = deltas.map(|(dx, (dy, dz))| [dx, dy, dz]).filter(|[dx, dy, dz]| !(*dx == 0 && *dy == 0 && *dz == 0));
    let cube_pos = (-dim..dim + 2).cartesian_product((-dim..dim + 2).cartesian_product(-dim..dim + 2).collect_vec());
    cube_pos.for_each(|(x, (y, z))| {
        let active = deltas.clone().filter(|[dx, dy, dz]| prev.contains(&[x + dx, y + dy, z + dz])).count();
        update_cube(&prev, &mut next, &[x, y, z], active);
    });
    next
}

fn step_4D(prev: &Grid4D) -> Grid4D {
    let mut next = prev.clone();
    let dim = *prev.iter().map(|[x, y, z, w]| *[x, y, z, w].iter().max().unwrap()).max().unwrap();
    let deltas = (-1..2).cartesian_product((-1..2).cartesian_product((-1..2).cartesian_product(-1..2)));
    let deltas = deltas.map(|(dx, (dy, (dz, dw)))| [dx, dy, dz, dw]).filter(|[dx, dy, dz, dw]| !(*dx == 0 && *dy == 0 && *dz == 0 && *dw == 0));
    let cube_pos = (-dim..dim + 2).cartesian_product((-dim..dim + 2).cartesian_product((-dim..dim + 2).cartesian_product(-dim..dim + 2)).collect_vec());
    cube_pos.for_each(|(x, (y, (z, w)))| {
        let active = deltas.clone().filter(|[dx, dy, dz, dw]| prev.contains(&[x + dx, y + dy, z + dz, w + dw])).count();
        update_cube(&prev, &mut next, &[x, y, z, w], active);
    });
    next
}

fn update_cube<T: Eq + Hash + Copy>(prev: &HashSet<T>, next: &mut HashSet<T>, key: &T, active: usize) {
    match prev.contains(key) {
        true if active != 2 && active != 3 => next.remove(key),
        false if active == 3 => next.insert(*key),
        _ => false,
    };
}
