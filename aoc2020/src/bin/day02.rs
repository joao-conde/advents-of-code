use regex::Regex;
use std::fs;

type Entry = (usize, usize, char, String);

fn main() {
    let input = fs::read_to_string("input/day02").expect("failure opening input file");
    let entries = parse_input(input);
    println!("Part1: {}", solve(&entries, policy1));
    println!("Part2: {}", solve(&entries, policy2));
}

fn solve(entries: &[Entry], policy: fn(&Entry) -> bool) -> usize {
    entries.iter().filter(|entry| policy(entry)).count()
}

fn policy1(entry: &Entry) -> bool {
    let (min, max, letter, password) = entry;
    let letter_cnt = password.chars().filter(|c| c == letter).count();
    letter_cnt >= *min && letter_cnt <= *max
}

fn policy2(entry: &Entry) -> bool {
    let (min, max, letter, password) = entry;
    (password.chars().nth(*min - 1) == Some(*letter))
        != (password.chars().nth(*max - 1) == Some(*letter))
}

fn parse_input(input: String) -> Vec<Entry> {
    fn parse_entry(entry: &str, re: &Regex) -> Entry {
        let matches = re.captures(entry).expect("wrong file entry format");
        (
            matches["min"]
                .parse::<usize>()
                .expect("min value should be a number"),
            matches["max"]
                .parse::<usize>()
                .expect("max value should be a number"),
            matches["letter"]
                .parse::<char>()
                .expect("letter should be one char"),
            matches["password"].to_owned(),
        )
    }
    let re = Regex::new(r"(?P<min>[0-9]*)-(?P<max>[0-9]*) (?P<letter>[a-zA-Z]): (?P<password>.*)")
        .expect("invalid regex expression");
    input
        .lines()
        .map(|x| parse_entry(x, &re))
        .collect::<Vec<Entry>>()
}
