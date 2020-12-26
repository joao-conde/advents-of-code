use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/day01").expect("failure opening input file");
    let nums = input
        .lines()
        .map(|x| x.parse().expect("input entry should be a number"))
        .collect::<HashSet<i32>>();
    println!("Part1: {}", p1(&nums, 2020));
    println!("Part2: {}", p2(&nums, 2020));
}

fn p1(nums: &HashSet<i32>, goal: i32) -> i32 {
    let (num1, num2) =
        find_target_pair(&nums, goal).expect("no two nums amount to the desired goal");
    num1 * num2
}

fn p2(nums: &HashSet<i32>, goal: i32) -> i32 {
    let (num1, num2) = nums
        .iter()
        .find_map(|num| find_target_pair(&nums, goal - num))
        .expect("no three nums amount to the desired goal");
    num1 * num2 * (goal - num1 - num2)
}

fn find_target_pair(nums: &HashSet<i32>, goal: i32) -> Option<(i32, i32)> {
    nums.iter()
        .find(|num| nums.contains(&(goal - *num)))
        .map(|num| (*num, goal - num))
}
