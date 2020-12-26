use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

const PREAMBLE_SIZE: usize = 25;

fn main() {
    let input = fs::read_to_string("input/day09").expect("failure opening input file");
    let nums = input
        .lines()
        .map(|num| num.parse().expect("not a number"))
        .collect::<Vec<i64>>();
    let p1_num = p1(&nums);
    println!("Part1: {}", p1_num);
    println!("Part2: {}", p2(&nums, p1_num));
}

fn p1(nums: &[i64]) -> i64 {
    (PREAMBLE_SIZE..nums.len())
        .find(|i| {
            let preamble: HashSet<&i64> = HashSet::from_iter(nums[i - PREAMBLE_SIZE..*i].iter());
            preamble
                .iter()
                .map(|p| nums[*i as usize] - *p)
                .all(|complement| !preamble.contains(&complement))
        })
        .map(|i| nums[i])
        .expect("no number violates the XMAS preamble rule")
}

fn p2(nums: &[i64], target: i64) -> i64 {
    let (mut i, mut j, mut sum) = (0, 0, 0);
    loop {
        match sum.cmp(&target) {
            Ordering::Greater => {
                sum -= nums[usize::min(i, nums.len() - 1)];
                i += 1
            }
            Ordering::Less => {
                sum += nums[usize::min(j, nums.len() - 1)];
                j += 1
            }
            Ordering::Equal => {
                return nums[i..j].iter().min().expect("iterator empty")
                    + nums[i..j].iter().max().expect("iterator empty")
            }
        }
    }
}
