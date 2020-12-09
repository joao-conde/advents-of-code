use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

const PREAMBLE_SIZE: usize = 25;

fn main() {
    let input = fs::read_to_string("input/day09").expect("failure opening input file");

    let nums = input.split("\n").map(|num| num.parse().expect("not a number")).collect::<Vec<i64>>();

    let p1_num = p1(&nums);
    println!("Part1: {}", p1_num);

    let (mut i, mut j) = (0, 2); 
    let mut cur_sum: i64 = nums[i..j].iter().sum();
    while cur_sum != p1_num {
        cur_sum = nums[i..j].iter().sum();        
        if cur_sum == p1_num {
            println!("Part2: {}", nums[i..j].iter().min().unwrap() + nums[i..j].iter().max().unwrap());
            break;
        } else if cur_sum > p1_num {
            i += 1;
        } else {
            j += 1;
        }
    }
}

fn p1(nums: &Vec<i64>) -> i64 {
    for i in PREAMBLE_SIZE..nums.len() {
        let num = nums[i];
        let preamble: HashSet<&i64> = HashSet::from_iter(nums[i-PREAMBLE_SIZE..i].iter());
        let b = preamble.iter().map(|p| num - *p).any(|complement| preamble.contains(&complement));
        if !b {
            return num;
        }
    }
    0
}
