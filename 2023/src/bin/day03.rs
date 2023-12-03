use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input/day03").unwrap();

    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let rows = lines.len();
    let cols = lines[0].len();

    let mut gear_ratios: Vec<usize> = vec![];
    let mut part_numbers = HashSet::new();
    for i in 0..rows {
        for j in 0..cols {
            if !lines[i][j].is_digit(10) && lines[i][j] != '.' {
                let neighbors = vec![
                    (i - 1, j - 1),
                    (i - 1, j),
                    (i - 1, j + 1),
                    (i, j - 1),
                    (i, j + 1),
                    (i + 1, j - 1),
                    (i + 1, j),
                    (i + 1, j + 1),
                ];

                let numbers: HashSet<(usize, usize, usize, usize)> = neighbors
                    .iter()
                    .filter(|(i, j)| lines.get(*i).and_then(|l| l.get(*j)).is_some())
                    .flat_map(|(i, j)| take_from(&lines, *i, *j))
                    .collect();

                part_numbers.extend(&numbers);

                if lines[i][j] == '*' && numbers.len() == 2 {
                    gear_ratios.push(numbers.iter().map(|(_, _, _, n)| n).product());
                }
            }
        }
    }

    let p1: usize = part_numbers.iter().map(|(_, _, _, n)| n).sum();
    let p2: usize = gear_ratios.iter().sum();
    println!("Part1: {p1}");
    println!("Part2: {p2}");
}

fn take_from(lines: &Vec<Vec<char>>, i: usize, j: usize) -> Option<(usize, usize, usize, usize)> {
    if !lines[i][j].is_digit(10) {
        return None;
    }

    let mut start = j;
    while start > 0 && lines[i][start - 1].is_digit(10) {
        start -= 1;
    }

    let mut end = j;
    while end < lines.len() - 1 && lines[i][end + 1].is_digit(10) {
        end += 1;
    }

    let number: String = lines[i][start..end + 1].iter().collect();
    let number = number.parse().ok();

    number.and_then(|n| Some((i, start, end, n)))
}
