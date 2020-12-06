use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/day06").expect("failure opening input file");
    let groups: Vec<Vec<&str>> = input.split("\n\n").map(|x| x.split('\n').collect()).collect();

    let groups_counts: Vec<HashMap<char, usize>> = groups.iter().map(|group| answers_map(group)).collect();

    let p1: usize = groups_counts.iter().map(HashMap::len).sum();
    println!("Part1: {:?}", p1);

    let groups_len = groups.iter().map(|group| group.len());
    let p2: usize = groups_counts.iter().zip(groups_len).map(count_all_yes).sum();
    println!("Part2: {:?}", p2);
}

fn answers_map<'a>(group: &'a [&str]) -> HashMap<char, usize> {
    group.iter().flat_map(|s| s.chars()).fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    })
}

fn count_all_yes((group_counts, group_len): (&HashMap<char, usize>, usize)) -> usize {
    group_counts.iter().filter(|(_, v)| **v == group_len).count()
}
