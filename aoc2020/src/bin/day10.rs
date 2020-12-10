use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/day10").expect("failure opening input file");

    let mut jolts = input.split("\n").flat_map(|line| line.parse()).collect::<Vec<u8>>();
    jolts.sort();
        
    jolts.insert(0, 0);
    jolts.push(jolts[jolts.len() - 1] + 3);

    let diffs = jolts[..].windows(2).map(|chunk| chunk[1] - chunk[0]).fold(HashMap::new(), |mut map, diff| {
        *map.entry(diff).or_insert(0) += 1;
        map
    });

    println!("Part1: {:?}", diffs[&1] * diffs[&3]);


    let mut dp = vec![0u64; jolts[jolts.len() - 1] as usize + 1]; //todo try remove
    dp[0] = 1;
    
    for jolt in jolts.iter().skip(1) {
        let jolt = *jolt;
        match jolt {
            1 => dp[jolt as usize] = dp[0],
            2 => dp[jolt as usize] = dp[0] + dp[1],
            _ => dp[jolt as usize] = dp[(jolt-1) as usize] + dp[(jolt-2)  as usize] + dp[(jolt-3)  as usize]
        }
    }

    println!("Part2: {}", dp[jolts[jolts.len() - 1] as usize]);

}
