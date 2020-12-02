use regex::Regex;
use std::fs;

const INPUT_PATH: &str = "input/day02";

type Entry = (usize, usize, char, String);

fn main() {
    let input = fs::read_to_string(INPUT_PATH).expect("failure opening input file");
    let entries = parse_input(input);
    println!("Part1: {}", part1(&entries));
    println!("Part2: {}", part2(&entries));
}

fn part1(entries: &[Entry]) -> i32 {
    let mut valid = 0;
    for entry in entries {
        let (min, max, letter, password) = entry;
        let letter_cnt = password.chars().filter(|c| c == letter).count();
        valid += if letter_cnt >= *min && letter_cnt <= *max { 1 } else { 0 };
    }
    valid
}

fn part2(entries: &[Entry]) -> i32 {
    let mut valid = 0;
    for entry in entries {
        let (min, max, letter, password) = entry;
        let is_valid = (password.chars().nth(*min - 1) == Some(*letter)) != (password.chars().nth(*max - 1) == Some(*letter));
        valid += if is_valid { 1 } else { 0 };
    }
    valid
}

fn parse_input(input: String) -> Vec<Entry> {
    fn parse_entry(entry: &str, re: &Regex) -> Entry {
        let matches = re.captures(entry).expect("wrong file entry format");
        (
            matches["min"].parse::<usize>().expect("min value should be a number"),
            matches["max"].parse::<usize>().expect("max value should be a number"),
            matches["letter"].parse::<char>().expect("letter should be one char"),
            matches["password"].to_owned(),
        )
    }

    let re = Regex::new("(?P<min>[0-9]*)-(?P<max>[0-9]*) (?P<letter>[a-zA-Z]): (?P<password>.*)").expect("invalid regex expression");

    input.split('\n').map(|x| parse_entry(x, &re)).collect::<Vec<Entry>>()
}

#[test]
fn examples() {
    let entries = parse_input("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc".to_owned());
    assert!(2 == part1(&entries));
    assert!(1 == part2(&entries));
}
