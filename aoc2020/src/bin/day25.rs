use std::fs;

fn main() {
    let pub_keys = fs::read_to_string("input/day25").expect("failure opening input file");
    let pub_door = pub_keys.lines().next().unwrap().parse::<usize>().unwrap();
    let pub_card = pub_keys.lines().nth(1).unwrap().parse::<usize>().unwrap();

    let mut x = 1;
    let mut loop_size = 0;
    while x != pub_card {
        x = x * 7 % 20201227;
        loop_size += 1;
    }

    let p1 = (0..loop_size).fold(1, |x, _| x * pub_door % 20201227);
    println!("Part1: {}", p1);
    println!("Part2: free 🌟!");
}
