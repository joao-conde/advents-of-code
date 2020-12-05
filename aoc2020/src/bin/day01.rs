use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/day01").expect("failure opening input file");
    let nums = parse_input(input);
    println!("Part1: {}", part1(&nums, 2020));
    println!("Part2: {}", part2(&nums, 2020));
}

fn part1(nums: &HashSet<i32>, goal: i32) -> i32 {
    let (e1, e2) = find_target_pair(&nums, goal).expect("no two nums amount to the desired goal");
    e1 * e2
}

fn part2(nums: &HashSet<i32>, goal: i32) -> i32 {
    for num in nums {
        let mut nums = nums.to_owned();
        nums.remove(num);
        if let Some((e1, e2)) = find_target_pair(&nums, goal - num) {
            return e1 * e2 * num;
        }
    }
    panic!("no three nums amount to the desired goal");
}

fn parse_input(input: String) -> HashSet<i32> {
    input.split('\n').map(|x| x.parse::<i32>().expect("input entry should be a number")).collect::<HashSet<i32>>()
}

fn find_target_pair(nums: &HashSet<i32>, goal: i32) -> Option<(i32, i32)> {
    for num in nums {
        let complement = goal - num;
        if nums.contains(&complement) {
            return Some((*num, complement));
        };
    }
    None
}
