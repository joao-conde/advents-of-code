fn main() {
    let input = std::fs::read_to_string("input/day06").unwrap();

    let (times, distances) = input.split_once('\n').unwrap();
    let (_, times_line) = times.split_once("Time:").unwrap();
    let (_, distances_line) = distances.split_once("Distance:").unwrap();

    let times = times_line
        .split(' ')
        .filter(|s| !s.is_empty())
        .flat_map(|s| s.parse::<usize>());
    let distances = distances_line
        .split(' ')
        .filter(|s| !s.is_empty())
        .flat_map(|s| s.parse::<usize>());

    let races = times.zip(distances);

    let p1: usize = races.map(|(t, d)| better_moves(t, d)).product();
    println!("Part1: {p1}");

    let time: usize = times_line
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .unwrap();
    let distance: usize = distances_line
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .unwrap();

    let p2: usize = better_moves(time, distance);
    println!("Part2: {p2}");
}

fn better_moves(time: usize, distance: usize) -> usize {
    (1..time)
        .map(|holding| holding * (time - holding))
        .filter(|d| *d > distance)
        .count()
}
