use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day03").unwrap();

    let banks = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let p1: usize = banks.iter().map(|bank| largest_joltage(bank, 2)).sum();
    println!("Part1: {p1}");

    let p2: usize = banks.iter().map(|bank| largest_joltage(bank, 12)).sum();
    println!("Part2: {p2}");
}

fn largest_joltage(bank: &[u32], size: u32) -> usize {
    if size == 0 {
        return 0;
    }

    let candidates_len = bank.len() - (size as usize - 1);
    let candidates = &bank[..candidates_len];

    let maximizing_joltage = *candidates.iter().max().unwrap();
    let maximizing_position = candidates
        .iter()
        .position(|joltage| *joltage == maximizing_joltage)
        .unwrap();

    let remaining = &bank[maximizing_position + 1..];

    maximizing_joltage as usize * 10usize.pow(size - 1) + largest_joltage(remaining, size - 1)
}
