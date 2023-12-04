use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input/day04").unwrap();

    let card_matches: Vec<usize> = input.lines().map(card_matches).collect();
    let mut card_counts = vec![1; card_matches.len()];
    for (i, matches) in card_matches.iter().enumerate() {
        for j in i + 1..=i + matches {
            if card_counts.get(j).is_none() {
                break;
            }
            card_counts[j] += card_counts[i];
        }
    }

    let p1: usize = card_matches.into_iter().map(card_score).sum();
    let p2: usize = card_counts.iter().sum();
    println!("Part1: {p1}");
    println!("Part2: {p2}");
}

fn card_matches(line: &str) -> usize {
    let (_, numbers) = line.split_once(':').unwrap();
    let (winning, owned) = numbers.split_once('|').unwrap();

    let winning_set: HashSet<usize> = winning.split(' ').flat_map(|n| n.parse()).collect();
    let owned_set: HashSet<usize> = owned.split(' ').flat_map(|n| n.parse()).collect();

    winning_set.intersection(&owned_set).count()
}

fn card_score(matches: usize) -> usize {
    if matches > 0 {
        (1..matches).fold(1, |acc, _| acc * 2)
    } else {
        0
    }
}
