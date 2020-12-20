use regex::Regex;
use std::fs;
use std::{collections::HashMap, str::Lines};

type Rules = HashMap<String, String>;

fn main() {
    let input = fs::read_to_string("input/day19").expect("failure opening input file");
    let mut rules: Rules = input.split("\n\n").next().unwrap().lines().fold(HashMap::new(), |mut acc, rule| {
        let lhs = rule.split(':').next().unwrap().trim().to_owned();
        let rhs = rule.split(':').nth(1).unwrap().trim().replace('"', "");
        acc.insert(lhs, rhs);
        acc
    });
    let messages = input.split("\n\n").nth(1).unwrap().lines();

    println!("Part1: {}", solve(&rules, messages.clone()));

    rules.insert("8".to_owned(), "42 | 42 42 | 42 42 42 | 42 42 42 42 | 42 42 42 42 42 | 42 42 42 42 42 42".to_owned());
    rules.insert("11".to_owned(), "42 31 | 42 42 31 31 | 42 42 42 31 31 31 | 42 42 42 42 31 31 31 31".to_owned());
    println!("Part2: {}", solve(&rules, messages));
}

fn solve(rules: &Rules, messages: Lines) -> usize {
    let re = format!("^{}$", expand(&rules, "0".to_owned()));
    let re = Regex::new(&re).unwrap();
    messages.filter(|msg| re.is_match(msg)).count()
}

fn expand(rules: &Rules, rule: String) -> String {
    let tokens = rules.get(&rule).unwrap_or_else(|| panic!("no such rule {}", rule));
    if !tokens.chars().next().unwrap().is_alphabetic() {
        let re = tokens
            .split(" | ")
            .map(|branch| {
                let mut pattern = branch.split(' ').map(|rule| expand(&rules, rule.to_owned())).collect::<String>();
                pattern.push('|');
                pattern
            })
            .collect::<String>();
        format!("({})", &re[..re.len() - 1])
    } else {
        tokens.to_owned()
    }
}
