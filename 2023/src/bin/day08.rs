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

fn p1(instructions: &Vec<char>, maps: &HashMap<&str, (&str, &str)>) -> usize {
    let mut i = 0;
    let mut cur = "AAA";
    while cur != "ZZZ" {
        let instruction = instructions[i % instructions.len()];
        let (left, right) = maps[cur];
        let next = match instruction {
            'L' => left,
            'R' => right,
            _ => unreachable!(),
        };
        cur = next;
        i += 1;
    }

    i
}

fn p2(instructions: &Vec<char>, maps: &HashMap<&str, (&str, &str)>) -> usize {
    let srcs: Vec<(&str, usize)> = maps
        .clone()
        .into_keys()
        .filter(|k| k.chars().last().unwrap() == 'A')
        .map(|k| (k, 0))
        .collect();

    let mut curs = srcs;
    let mut found = vec![];
    while let Some((node, cnt)) = curs.pop() {
        if node.chars().last().unwrap() == 'Z' {
            found.push((node, cnt))
        } else {
            let instruction = instructions[cnt % instructions.len()];
            let next = match instruction {
                'L' => maps[node].0,
                'R' => maps[node].1,
                _ => unreachable!(),
            };
            curs.push((next, cnt + 1));
        }
    }

    found
        .iter()
        .map(|(_, cnt)| cnt)
        .fold(found[0].1, |acc, cnt| lcm(acc, *cnt))
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
