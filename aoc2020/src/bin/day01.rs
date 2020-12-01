use std::collections::HashSet;
use std::fs;

const GOAL: i32 = 2020;
const INPUT_PATH: &str = "input/day01";

fn main() {
    let entries = fs::read_to_string(INPUT_PATH)
        .expect("failure opening input file")
        .split("\n")
        .map(|x| x.parse::<i32>().expect("input entry should be a number"))
        .collect::<HashSet<i32>>();

    println!("Part1: {}", part1(&entries, GOAL));
    println!("Part2: {}", part2(&entries, GOAL));
}

fn part1(entries: &HashSet<i32>, goal: i32) -> i32 {
    let (e1, e2) = find_target_pair(&entries, goal).expect("no two entries amount to the desired goal");
    e1 * e2
}

fn part2(entries: &HashSet<i32>, goal: i32) -> i32 {
    for num in entries {
        let mut entries = entries.to_owned();
        entries.remove(num);
        if let Some((e1, e2)) = find_target_pair(&entries, goal - num) {
            return e1 * e2 * num;
        }
    }
    panic!("no three entries amount to the desired goal");
}

fn find_target_pair(entries: &HashSet<i32>, goal: i32) -> Option<(i32, i32)> {
    for num in entries {
        let complement = goal - num;
        if entries.contains(&complement) {
            return Some((*num, complement));
        };
    }
    None
}

#[test]
fn p1_example() {
    let entries = vec![1721, 979, 366, 299, 675, 1456]
        .into_iter()
        .collect::<HashSet<i32>>();
    let expected = 514579;
    let actual = part1(&entries, GOAL);
    assert!(expected == actual);
}

#[test]
fn p2_example() {
    let entries = vec![1721, 979, 366, 299, 675, 1456]
        .into_iter()
        .collect::<HashSet<i32>>();
    let expected = 241861950;
    let actual = part2(&entries, GOAL);
    assert!(expected == actual);
}
