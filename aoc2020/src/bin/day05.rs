use std::fs;

fn main() {
    let input = fs::read_to_string("input/day05").expect("failure opening input file");
    let seats = input.split('\n').map(|seat| seat_id(&seat)).collect::<Vec<i32>>();

    let p1 = seats.iter().max().expect("there are no seats");
    println!("Part1: {}", p1);

    let p2 = seats
        .iter()
        .filter(|seat| !seats.contains(&(*seat + 1)) && seats.contains(&(*seat + 2)))
        .map(|x| x + 1)
        .next()
        .expect("no seat available");
    println!("Part2: {}", p2);
}

fn seat_id(input: &str) -> i32 {
    let row = guided_binary_search(&input[0..7], 'F', 'B', 0, 127);
    let col = guided_binary_search(&input[7..10], 'L', 'R', 0, 7);
    row * 8 + col
}

fn guided_binary_search(instr: &str, left: char, right: char, mut lb: i32, mut ub: i32) -> i32 {
    let mut mid = lb + (ub - lb) / 2;
    for c in instr.chars() {
        if c == left {
            ub = mid - 1
        } else if c == right {
            lb = mid + 1
        }
        mid = lb + (ub - lb) / 2;
    }
    mid
}
