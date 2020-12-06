use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

fn main() {
    let input = fs::read_to_string("input/day06").expect("failure opening input file");
    let groups: Vec<Vec<HashSet<char>>> =
        input.split("\n\n").map(|group| group.split('\n').map(|vote| HashSet::from_iter(vote.chars())).collect()).collect();

    let p1: usize = groups.iter().map(|group| union_sets(group)).map(|s| s.len()).sum();
    println!("Part1: {:?}", p1);

    let p2: usize = groups.iter().map(|group| intersect_sets(group)).map(|s| s.len()).sum();
    println!("Part2: {:?}", p2);
}

fn union_sets(sets: &[HashSet<char>]) -> HashSet<char> {
    sets.iter().fold(HashSet::new(), |s1: HashSet<char>, s2| s1.union(&s2).copied().collect())
}

fn intersect_sets(sets: &[HashSet<char>]) -> HashSet<char> {
    sets.iter().fold(sets[0].clone(), |s1: HashSet<char>, s2| s1.intersection(&s2).copied().collect())
}
