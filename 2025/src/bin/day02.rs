use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day02").unwrap();

    let ranges: Vec<(usize, usize)> = input
        .split(",")
        .map(|range| {
            let (from, to) = range.split_once("-").unwrap();
            (from.parse().unwrap(), to.parse().unwrap())
        })
        .collect();

    let p1: usize = ranges
        .iter()
        .flat_map(|&(from, to)| (from..=to).filter(|id| is_repeated_n_times(&id.to_string(), 2)))
        .sum();
    println!("Part1: {p1}");

    let p2: usize = ranges
        .iter()
        .flat_map(|&(from, to)| (from..=to).filter(|id| is_repeated(&id.to_string())))
        .sum();
    println!("Part2: {p2}");
}

fn is_repeated(id: &str) -> bool {
    for n in 2..=id.len() {
        if is_repeated_n_times(id, n) {
            return true;
        }
    }
    false
}

fn is_repeated_n_times(id: &str, n: usize) -> bool {
    let pattern_len = id.len() / n;
    if pattern_len == 0 {
        return false;
    }

    let pattern = &id[..pattern_len];
    let chunks = id.chars().chunks(pattern.len());

    if chunks.clone().into_iter().count() != n {
        return false;
    }

    chunks
        .into_iter()
        .all(|chunk| String::from_iter(chunk) == pattern)
}
