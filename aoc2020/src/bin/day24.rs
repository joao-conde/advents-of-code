use std::fs;
use std::collections::HashSet;


fn main() {
    let input = fs::read_to_string("input/day24").expect("failure opening input file");

    let (mut blacks, mut whites) = place_tiles(&input);
    println!("Part1: {}", blacks.len());

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
    println!("Part1: {}", blacks.len());
}

fn place_tiles(input: &str) -> (HashSet<(i32, i32)>, HashSet<(i32, i32)>) {
    let mut blacks = HashSet::new();
    let mut whites = HashSet::new();
    for line in input.lines() {
        let moves = parse_move(line);
        let tile = identify_tile(&moves);
        if blacks.contains(&tile) {
            blacks.remove(&tile);
            whites.insert(tile);
        } else {
            whites.remove(&tile);
            blacks.insert(tile);
        }
    }
    (blacks, whites)
}

fn identify_tile(moves: &Vec<&str>) -> (i32, i32) {
    let mut coords = (0, 0);
    for m in moves {
        match *m {
            "nw" => coords.1 -= 1,
            "ne" => {coords.0 += 1; coords.1 -= 1}
            "sw" => {coords.0 -= 1; coords.1 += 1}
            "se" => coords.1 += 1,
            "w" => coords.0 -= 1,
            "e" => coords.0 += 1,
            _ => ()
        }
    }
    coords
}

fn parse_move(move_str: &str) -> Vec<&str> {
    let mut moves = vec![];
    let mut i = 0;
    while i < move_str.len() {
        if i == move_str.len() - 1 {
            moves.push(&move_str[i..i+1]);
            i += 1;
        } else {
            match &move_str[i..i+2] {
                "nw" | "ne" | "sw" | "se" => {
                    moves.push(&move_str[i..i+2]);
                    i += 2;
                },
                _ => {
                    moves.push(&move_str[i..i+1]);
                    i += 1;
                }
            } 
        }
    }
    moves
}