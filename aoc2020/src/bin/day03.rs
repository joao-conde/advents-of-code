use std::fs;

const INPUT_PATH: &str = "input/day03";
const TREE_CHAR: char = '#';

fn main() {
    let input = fs::read_to_string(INPUT_PATH).expect("failure opening input file");
    let map = input.split('\n').map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();
    println!("Part1: {}", trees_hit(&map, (3, 1)));
    println!("Part2: {}", part2(&map));
}

fn part2(map: &[Vec<char>]) -> u32 {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes.iter().map(|slope| trees_hit(&map, *slope)).product()
}

fn trees_hit(map: &[Vec<char>], slope: (usize, usize)) -> u32 {
    let mut trees_hit = 0;
    let (mut x, mut y) = (0, 0);
    let (width, height) = (map[0].len(), map.len());
    while y < height {
        trees_hit += if map[y][x] == TREE_CHAR { 1 } else { 0 };
        x += slope.0;
        y += slope.1;
        x %= width;
    }
    trees_hit
}

#[test]
fn examples() {
    let input = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";
    let map = input.split('\n').map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();
    assert!(trees_hit(&map, (3, 1)) == 7);
    assert!(part2(&map) == 336);
}

#[test]
fn puzzle() {
    let input = fs::read_to_string(INPUT_PATH).expect("failure opening input file");
    let map = input.split('\n').map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();
    assert!(trees_hit(&map, (3, 1)) == 184);
    assert!(part2(&map) == 2431272960);
}
