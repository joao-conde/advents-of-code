use std::fs;

fn main() {
    let pub_keys = fs::read_to_string("input/day25").expect("failure opening input file");
    let pub_door = pub_keys.lines().next().unwrap().parse().unwrap();
    let pub_card = pub_keys.lines().nth(1).unwrap().parse().unwrap();

    let mut x = 1;
    let mut loop_size = 0;
    while x != pub_card {
        x = x * 7 % 20201227;
        loop_size += 1;
    }

    let p1 = transform(loop_size, pub_door);
    println!("Part1: {}", p1);
    println!("Part2: free 🌟!");
}

fn transform(loop_size: usize, subject: usize) -> usize {
    let mut value = 1;
    for _ in 0..loop_size {
        value *= subject;
        value %= 20201227;
    }
    value
}
