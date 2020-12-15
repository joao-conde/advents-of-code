use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/day15").expect("failure opening input file");
    let nums = input.split(',').map(|num| num.parse().unwrap()).collect::<Vec<usize>>();
    println!("Part1: {}", mem_game(&nums, 2020));
    println!("Part2: {}", mem_game(&nums, 30000000));
}

fn mem_game(nums: &[usize], stop: usize) -> usize {
    let mut history = nums.iter().take(nums.len() - 1).enumerate().fold(HashMap::new(), |mut hist, (i, n)| {
        hist.insert(*n, i);
        hist
    });

    let mut last = *nums.iter().last().unwrap();
    for turn in nums.len()..stop {
        let next = match history.get(&last) {
            Some(pos) => turn - 1 - pos,
            None => 0,
        };
        history.insert(last, turn - 1);
        last = next;
    }
    last
}
