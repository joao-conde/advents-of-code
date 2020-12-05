use std::fs;

fn main() {
    let input = fs::read_to_string("input/day05").expect("failure opening input file");
    let mut seats = input.split('\n').map(|seat| seat_id(&seat)).collect::<Vec<i32>>();

    let p1 = seats.iter().max().expect("there are no seats");
    println!("Part1: {}", p1);

    seats.sort();
    let p2 = seats.windows(2).filter(|seats| seats[1] - seats[0] == 2).next().expect("no seat available")[0] + 1;
    println!("Part2: {}", p2);
}

fn seat_id(input: &str) -> i32 {
    let row = input[0..7].replace("F", "0").replace("B", "1");
    let row = i32::from_str_radix(&row, 2).expect("invalid seat string");
    let col = input[7..10].replace("L", "0").replace("R", "1");
    let col = i32::from_str_radix(&col, 2).expect("invalid seat string");
    row * 8 + col
}
