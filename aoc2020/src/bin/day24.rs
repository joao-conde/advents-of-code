use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/day24").expect("failure opening input file");
    let blacks = p1(&input);
    println!("Part1: {}", blacks.len());
    println!("Part2: {}", p2(blacks));
}

fn p1(input: &str) -> HashSet<(i32, i32)> {
    let mut blacks = HashSet::new();
    for line in input.lines() {
        let moves = parse_move(line);
        let tile = identify_tile(&moves);
        if blacks.contains(&tile) {
            blacks.remove(&tile);
        } else {
            blacks.insert(tile);
        }
    }
    blacks
}

fn p2(mut blacks: HashSet<(i32, i32)>) -> usize {
    let mut whites = HashSet::new();
    for x in -100..100 {
        for y in -100..100 {
            if !blacks.contains(&(x, y)) {
                whites.insert((x, y));
            }
        }
    }

    let deltas = [(0, -1), (1, -1), (1, 0), (0, 1), (-1, 1), (-1, 0)];
    for _ in 0..100 {
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
    }
    blacks.len()
}

fn identify_tile(moves: &[&str]) -> (i32, i32) {
    let mut coords = (0, 0);
    for m in moves {
        match *m {
            "nw" => coords.1 -= 1,
            "ne" => {
                coords.0 += 1;
                coords.1 -= 1
            }
            "sw" => {
                coords.0 -= 1;
                coords.1 += 1
            }
            "se" => coords.1 += 1,
            "w" => coords.0 -= 1,
            "e" => coords.0 += 1,
            _ => panic!("invalid move"),
        }
    }
    coords
}

fn parse_move(move_str: &str) -> Vec<&str> {
    let mut i = 0;
    let mut moves = vec![];
    while i < move_str.len() {
        if i == move_str.len() - 1 {
            moves.push(&move_str[i..i + 1]);
            i += 1;
            continue;
        }
        match &move_str[i..i + 2] {
            "nw" | "ne" | "sw" | "se" => {
                moves.push(&move_str[i..i + 2]);
                i += 2;
            }
            _ => {
                moves.push(&move_str[i..i + 1]);
                i += 1;
            }
        }
    }
    moves
}
