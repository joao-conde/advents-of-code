use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/day24").expect("failure opening input file");
    let blacks = p1(&input);
    println!("Part1: {}", blacks.len());
    println!("Part2: {}", p2(blacks));
}

fn p1(input: &str) -> HashSet<(i32, i32)> {
    input.lines().map(|line| identify_tile(&line)).fold(HashSet::new(), |mut blacks, tile| {
        if blacks.contains(&tile) {
            blacks.remove(&tile);
        } else {
            blacks.insert(tile);
        }
        blacks
    })
}

fn p2(mut blacks: HashSet<(i32, i32)>) -> usize {
    let mut whites = (-100..100)
        .map(|x| (-100..100).map(|y| (x, y)).collect::<Vec<(i32, i32)>>())
        .flatten()
        .filter(|p| !blacks.contains(p))
        .fold(HashSet::new(), |mut whites, p| {
            whites.insert(p);
            whites
        });

    let deltas = [(0, -1), (1, -1), (1, 0), (0, 1), (-1, 1), (-1, 0)];
    (0..100).for_each(|_| {
        let mut next_blacks = HashSet::new();
        let mut next_whites = HashSet::new();
        for b in &blacks {
            let neighbors = deltas.iter().map(|d| (b.0 + d.0, b.1 + d.1)).filter(|n| blacks.contains(n)).count();
            if neighbors == 0 || neighbors > 2 {
                next_whites.insert(*b);
            } else {
                next_blacks.insert(*b);
            }
        }
        for w in &whites {
            let neighbors = deltas.iter().map(|d| (w.0 + d.0, w.1 + d.1)).filter(|n| blacks.contains(n)).count();
            if neighbors == 2 {
                next_blacks.insert(*w);
            } else {
                next_whites.insert(*w);
            }
        }
        blacks = next_blacks;
        whites = next_whites;
    });
    blacks.len()
}

fn identify_tile(moves: &str) -> (i32, i32) {
    let mut i = 0;
    let mut coords = (0, 0);
    while i < moves.len() {
        match moves.get(i..i + 2) {
            Some("nw") => {
                coords.1 -= 1;
                i += 2
            }
            Some("ne") => {
                coords.0 += 1;
                coords.1 -= 1;
                i += 2
            }
            Some("sw") => {
                coords.0 -= 1;
                coords.1 += 1;
                i += 2
            }
            Some("se") => {
                coords.1 += 1;
                i += 2
            }
            _ => match moves.get(i..i + 1) {
                Some("w") => {
                    coords.0 -= 1;
                    i += 1
                }
                Some("e") => {
                    coords.0 += 1;
                    i += 1
                }
                _ => panic!("unrecognized direction"),
            },
        }
    }
    coords
}
