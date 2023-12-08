use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input/day08").unwrap();
    let (instructions, maps) = input.split_once("\n\n").unwrap();
    let instructions: Vec<char> = instructions.chars().collect();
    let maps: HashMap<&str, (&str, &str)> = maps
        .lines()
        .map(|l| {
            let (src, dsts) = l.split_once(" = ").unwrap();
            let (left, right) = dsts.split_once(", ").unwrap();
            let left = &left[1..];
            let right = &right[..right.len() - 1];
            (src, (left, right))
        })
        .collect();

    let p1 = p1(&instructions, &maps);
    println!("Part1: {p1}");

    let p2 = p2(&instructions, &maps);
    println!("Part2: {p2}");
}

fn p1(instructions: &[char], maps: &HashMap<&str, (&str, &str)>) -> usize {
    navigate(&["AAA"], &["ZZZ"], instructions, maps)
}

fn p2(instructions: &[char], maps: &HashMap<&str, (&str, &str)>) -> usize {
    let srcs: Vec<&str> = maps
        .clone()
        .into_keys()
        .filter(|k| k.ends_with('A'))
        .collect();
    let dsts: Vec<&str> = maps
        .clone()
        .into_keys()
        .filter(|k| k.ends_with('Z'))
        .collect();
    navigate(&srcs, &dsts, instructions, maps)
}

fn navigate(
    srcs: &[&str],
    dsts: &[&str],
    instructions: &[char],
    maps: &HashMap<&str, (&str, &str)>,
) -> usize {
    let steps: Vec<usize> = srcs
        .iter()
        .map(|s| steps(s, dsts, instructions, maps))
        .collect();
    steps.iter().fold(steps[0], |acc, s| lcm(acc, *s))
}

fn steps<'a>(
    src: &'a str,
    dsts: &[&str],
    instructions: &[char],
    maps: &HashMap<&str, (&'a str, &'a str)>,
) -> usize {
    let mut cur = src;
    let mut steps = 0;
    while !dsts.contains(&cur) {
        let instruction = &instructions[steps % instructions.len()];
        let next = match instruction {
            'L' => maps[cur].0,
            'R' => maps[cur].1,
            _ => unreachable!(),
        };
        cur = next;
        steps += 1;
    }
    steps
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = usize::max(first, second);
    let mut min = usize::min(first, second);

    loop {
        let remainder = max % min;
        if remainder == 0 {
            return min;
        }

        max = min;
        min = remainder;
    }
}
