use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input/day04").unwrap();

    let mut paper_positions: HashSet<(isize, isize)> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '@')
                .map(|(j, _)| (i as isize, j as isize))
                .collect_vec()
        })
        .collect();

    let p1: usize = accessable_paper_positions(&paper_positions).len();
    println!("Part1: {p1}");

    let mut p2 = 0;
    loop {
        let accessible = accessable_paper_positions(&paper_positions);
        if accessible.is_empty() {
            break;
        }

        p2 += accessible.len();

        for (i, j) in accessible {
            paper_positions.remove(&(i, j));
        }
    }
    println!("Part2: {p2}");
}

fn accessable_paper_positions(paper_positions: &HashSet<(isize, isize)>) -> Vec<(isize, isize)> {
    paper_positions
        .iter()
        .filter(|(i, j)| is_acessable(paper_positions, *i, *j))
        .map(|(i, j)| (*i, *j))
        .collect_vec()
}

fn is_acessable(paper_positions: &HashSet<(isize, isize)>, i: isize, j: isize) -> bool {
    [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ]
    .iter()
    .filter(|(di, dj)| paper_positions.contains(&(i + di, j + dj)))
    .count()
        < 4
}
