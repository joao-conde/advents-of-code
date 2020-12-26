use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/day10").expect("failure opening input file");
    let mut jolts = input.lines().flat_map(|line| line.parse()).collect::<Vec<usize>>();
    jolts.sort();
    jolts.insert(0, 0);
    jolts.push(jolts[jolts.len() - 1] + 3);

    let diffs = jolts[..].windows(2).map(|chunk| chunk[1] - chunk[0]).fold(HashMap::new(), |mut map, diff| {
        *map.entry(diff).or_insert(0) += 1;
        map
    });
    println!("Part1: {}", diffs[&1] * diffs[&3]);

    // Part 2 example (sum last 3 numbers, not positions)
    // 0 1 4 5 6 7 10 11 12 15 16 19 22
    // 1 1 1 1 2 4  4  4  8  8  8  8  8
    let max = jolts[jolts.len() - 1];
    let mut dp = vec![0u64; max + 1];
    for jolt in jolts {
        match jolt {
            0 => dp[jolt] = 1,
            1 => dp[jolt] = 1,
            2 => dp[jolt] += dp[jolt - 1] + dp[jolt - 2],
            _ => dp[jolt] = dp[jolt - 1] + dp[jolt - 2] + dp[jolt - 3],
        }
    }
    println!("Part2: {}", dp[max]);
}
