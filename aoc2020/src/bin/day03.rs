use std::fs;

fn main() {
    let input = fs::read_to_string("input/day03").expect("failure opening input file");
    let map = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    println!("Part1: {}", trees_hit(&map, (3, 1)));
    println!("Part2: {}", part2(&map));
}

fn part2(map: &[Vec<char>]) -> u32 {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|slope| trees_hit(&map, *slope))
        .product()
}

fn trees_hit(map: &[Vec<char>], slope: (usize, usize)) -> u32 {
    let (mut x, mut y, mut trees_hit) = (0, 0, 0);
    let (width, height) = (map[0].len(), map.len());
    while y < height {
        trees_hit += if map[y][x] == '#' { 1 } else { 0 };
        x = (x + slope.0) % width;
        y += slope.1;
    }
    trees_hit
}
