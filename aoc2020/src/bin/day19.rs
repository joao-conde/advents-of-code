use regex::Regex;
use std::fs;
use std::{collections::HashMap, str::Lines};

type Rules = HashMap<String, String>;
fn main() {
    let input = fs::read_to_string("input/day19").expect("failure opening input file");
    let rules: Rules = input.split("\n\n").next().unwrap().lines().fold(HashMap::new(), |mut acc, rule| {
        let lhs = rule.split(':').next().unwrap().trim().to_owned();
        let rhs = rule.split(':').nth(1).unwrap().trim().replace('"', "");
        acc.insert(lhs, rhs);
        acc
    });
    let messages = input.split("\n\n").nth(1).unwrap().lines();
    println!("Part1: {}", p1(&rules, messages.clone()));
    println!("Part2: {}", p2(rules, messages));
}

fn p1(rules: &Rules, messages: Lines) -> usize {
    let re = format!("^{}$", expand(&rules, "0".to_owned()));
    let re = Regex::new(&re).unwrap();
    messages.filter(|msg| re.is_match(msg)).count()
}

fn p2(mut rules: Rules, messages: Lines) -> usize {
    rules.insert("8".to_owned(), "42 | 42 42 | 42 42 42 | 42 42 42 42 | 42 42 42 42 42 | 42 42 42 42 42 42".to_owned());
    rules.insert("11".to_owned(), "42 31 | 42 42 31 31 | 42 42 42 31 31 31 | 42 42 42 42 31 31 31 31".to_owned());
    let re = format!("^{}$", expand(&rules, "0".to_owned()));
    let re = Regex::new(&re).unwrap();
    messages.filter(|msg| re.is_match(msg)).count()
}

fn expand(rules: &Rules, rule: String) -> String {
    let tokens = rules.get(&rule).unwrap_or_else(|| panic!("no such rule {}", rule));
    
    if tokens.chars().next().unwrap().is_alphabetic() {
        return tokens.clone();
    }

    let mut string = String::from("(");
    let l = tokens
        .split(" | ")
        .map(|branch| {
            let mut pattern = branch.split(' ').map(|rule| expand(&rules, rule.to_owned())).collect::<String>();
            pattern.push('|');
            pattern
        })
        .collect::<String>();

    string.push_str(&l);
    string.pop();
    string.push(')');
    string
}
