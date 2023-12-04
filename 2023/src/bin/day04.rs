use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("input/day04").unwrap();

    let card_matches: Vec<usize> = input.lines().map(card_matches).collect();
    let scores: Vec<usize> = card_matches
        .iter()
        .map(|&matches| score_card(matches))
        .collect();

    let mut multipliers: Vec<usize> = vec![1; card_matches.len()];
    for (i, matches) in card_matches.iter().enumerate() {
        for j in i + 1..=i + matches {
            if let Some(_) = multipliers.get(j) {
                multipliers[j] += multipliers[i];
            }
        }
    }

    let p1: usize = scores.iter().sum();
    let p2: usize = multipliers.iter().sum();
    println!("Part1: {p1}");
    println!("Part2: {p2}");
}

fn card_matches(line: &str) -> usize {
    let (_, numbers) = line.split_once(':').unwrap();

    let (winning, owned) = numbers.split_once('|').unwrap();

    let winning_set: HashSet<usize> = winning
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|n| n.parse().unwrap())
        .collect();
    let owned_set: HashSet<usize> = owned
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|n| n.parse().unwrap())
        .collect();

    winning_set.intersection(&owned_set).count()
}

fn score_card(matches: usize) -> usize {
    if matches > 0 {
        (1..matches).fold(1, |acc, _| acc * 2)
    } else {
        0
    }
}
