use std::fs;

fn main() {
    let input = fs::read_to_string("input/day05").expect("failure opening input file");
    let seats = input.split("\n");

    let p1 = seats.clone().map(|seat| seat_id(&seat)).max().unwrap();
    println!("Part1: {}", p1);

    let mut ids = vec![];
    for r in 1..126 {
        for c in 0..7 {
            ids.push(r * 8 + c);
        }
    }

    let seats = seats.map(|seat| seat_id(&seat)).collect::<Vec<i32>>();
    for p in &seats {
        if !seats.contains(&&(*p + 1)) && seats.contains(&&(*p + 2)) {
            println!("Part2: {}", *p + 1);
        }
    }
}

fn seat_id(input: &str) -> i32 {
    let (mut lb, mut ub) = (0 as i32, 127 as i32);
    let mut row = lb + (ub - lb) / 2;
    for c in input[0..7].chars() {
        match c {
            'F' => ub = row - 1,
            'B' => lb = row + 1,
            _ => panic!(),
        }
        row = lb + (ub - lb) / 2;
    }

    let (mut lb, mut ub) = (0 as i32, 7 as i32);
    let mut col = lb + (ub - lb) / 2;
    for c in input[7..10].chars() {
        match c {
            'L' => ub = col - 1,
            'R' => lb = col + 1,
            _ => panic!(),
        }
        col = lb + (ub - lb) / 2;
    }

    row * 8 + col
}
