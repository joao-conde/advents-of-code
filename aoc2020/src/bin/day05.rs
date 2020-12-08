use std::fs;

fn main() {
    let input = fs::read_to_string("input/day05").expect("failure opening input file");
    let mut seats = input.split('\n').map(|seat| seat_id(&seat)).collect::<Vec<i32>>();

    seats.sort();
    println!("Part1: {}", seats.last().expect("zero seat IDs provided"));

    let p2 = seats.windows(2).find(|seats| seats[1] - seats[0] == 2).expect("no seat available")[0] + 1;
    println!("Part2: {}", p2);
}

fn seat_id(input: &str) -> i32 {
    let row = input[0..7].replace("F", "0").replace("B", "1");
    let row = i32::from_str_radix(&row, 2).expect("invalid seat string");
    let col = input[7..10].replace("L", "0").replace("R", "1");
    let col = i32::from_str_radix(&col, 2).expect("invalid seat string");
    row * 8 + col
}
