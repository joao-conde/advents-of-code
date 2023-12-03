use std::collections::HashSet;

type Part = (usize, usize, usize, usize);

fn main() {
    let input = std::fs::read_to_string("input/day03").unwrap();
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut gear_ratios = vec![];
    let mut parts = HashSet::new();
    for (i, line) in lines.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if !is_symbol(*char) {
                continue;
            }

            let neighbors = [
                (i - 1, j - 1),
                (i - 1, j),
                (i - 1, j + 1),
                (i, j - 1),
                (i, j + 1),
                (i + 1, j - 1),
                (i + 1, j),
                (i + 1, j + 1),
            ];

            let numbers: HashSet<Part> = neighbors
                .iter()
                .filter(|(i, j)| in_bounds(&lines, *i, *j))
                .flat_map(|(i, j)| part_from(&lines, *i, *j))
                .collect();

            parts.extend(&numbers);

            if is_gear(*char, numbers.len()) {
                gear_ratios.push(numbers.iter().map(|(_, _, _, n)| n).product());
            }
        }
    }

    let p1: usize = parts.iter().map(|(_, _, _, n)| n).sum();
    let p2: usize = gear_ratios.iter().sum();
    println!("Part1: {p1}");
    println!("Part2: {p2}");
}

fn part_from(lines: &Vec<Vec<char>>, i: usize, j: usize) -> Option<Part> {
    if !lines[i][j].is_ascii_digit() {
        return None;
    }

    let mut start = j;
    while start > 0 && lines[i][start - 1].is_ascii_digit() {
        start -= 1;
    }

    let mut end = j;
    while end < lines.len() - 1 && lines[i][end + 1].is_ascii_digit() {
        end += 1;
    }

    let number: String = lines[i][start..end + 1].iter().collect();
    let number: usize = number.parse().ok()?;
    Some((i, start, end, number))
}

fn in_bounds(lines: &[Vec<char>], i: usize, j: usize) -> bool {
    lines.get(i).and_then(|l| l.get(j)).is_some()
}

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

fn is_gear(c: char, adjacent: usize) -> bool {
    c == '*' && adjacent == 2
}
