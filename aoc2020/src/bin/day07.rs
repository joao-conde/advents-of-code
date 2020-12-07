use regex::Regex;
use std::collections::HashMap;
use std::fs;

type BagRules = HashMap<String, Vec<(u32, String)>>;

fn main() {
    let input = fs::read_to_string("input/day07").expect("failure opening input file");
    let map = parse_input(&input);
    let my_bag = "shiny gold";
    let p1 = map.keys().filter(|key| **key != my_bag).map(|key| closure(key, &map)).filter(|closure| closure.contains(&my_bag)).count();
    println!("Part1: {}", p1);
    println!("Part2: {}", num_bags(my_bag, &map) - 1);
}

fn closure<'a>(bag: &'a str, map: &'a BagRules) -> Vec<&'a str> {
    map.get(bag)
        .map(|bags| {
            bags.iter().fold(vec![bag], |mut acc, (_, bag)| {
                acc.append(&mut closure(bag, &map));
                acc
            })
        })
        .unwrap_or_else(Vec::new)
}

fn num_bags(bag: &str, map: &BagRules) -> u32 {
    map.get(bag)
        .map(|bags| {
            bags.iter().fold(1, |mut acc, (quant, bag)| {
                acc += quant * num_bags(bag, &map);
                acc
            })
        })
        .unwrap_or(1)
}

fn parse_input(input: &str) -> BagRules {
    let bag_re = Regex::new(r"^(?P<bag>\w+ \w+) bags contain").unwrap();
    let contents_re = Regex::new(r"(?P<quantity>[0-9]+) (?P<bag>\w+ \w+)").unwrap();
    let mut rules = HashMap::new();
    for line in input.lines() {
        let bag = bag_re.captures(line).unwrap()["bag"].to_string();
        for capture in contents_re.captures_iter(line) {
            let child_bag = capture["bag"].to_string();
            let quantity = capture["quantity"].parse().unwrap();
            rules.entry(bag.clone()).or_insert_with(Vec::new).push((quantity, child_bag));
        }
    }
    rules
}
