fn main() {
    let input = std::fs::read_to_string("input/day09").unwrap();
    let report: Vec<Vec<isize>> = input
        .lines()
        .map(|l| l.split_whitespace().map(|x| x.parse().unwrap()).collect())
        .collect();

    let p1: isize = report.iter().map(expand).map(extrapolate_forwards).sum();
    println!("Part1: {p1}");

    let p2: isize = report.iter().map(expand).map(extrapolate_backwards).sum();
    println!("Part2: {p2}");
}

fn expand(history: &Vec<isize>) -> Vec<Vec<isize>> {
    let mut sequence = history.clone();
    let mut sequences = vec![sequence.clone()];

    while !sequence.iter().all(|x| *x == 0) {
        let new_sequence: Vec<isize> = sequence.windows(2).map(|w| w[1] - w[0]).collect();
        sequences.push(new_sequence.clone());
        sequence = new_sequence;
    }

    sequences
}

fn extrapolate_forwards(sequences: Vec<Vec<isize>>) -> isize {
    sequences
        .iter()
        .rev()
        .fold(0, |last, seq| seq.last().unwrap() + last)
}

fn extrapolate_backwards(sequences: Vec<Vec<isize>>) -> isize {
    sequences
        .iter()
        .rev()
        .fold(0, |last, seq| seq.iter().rev().last().unwrap() - last)
}
